extern crate csv;
extern crate mongodb;
extern crate select;
use bson::{bson, doc};
use csv::Writer;
use mongodb::{ options::ClientOptions, Client };
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::process;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Record {
    id: String,
    title: String,
    description: String,
    url: String,
    street_address: String,
    locality: String,
    region: String,
    postal_code: String,
    country: String,
    latitude: String,
    longitude: String,
    email: String,
    telephone: String,
    events_count: String,
}

#[tokio::main]
async fn scraping() -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://calagator.org/venues.json").await?;
    let status = response.status();
    let body = response.text().await?;
    // println!("{:?}", status);
    // println!("{:?}", body);
    let mut file = File::create("assets/venues.json")?;
    file.write_all(body.as_bytes())?;
    file.sync_all()?;
    Ok(status)
}

fn scraping_event() {
    let html = scraping();
    match html {
        Ok(v) => {
            println!("\nResponse code: {}", v);
            println!("Successfully scraped venues to assets/venues.json");
        }
        Err(e) => println!("error scraping events: {}", e),
    }
    process::exit(1);
}

#[allow(bare_trait_objects)]
fn parse_date() -> Result<String, Box<Error>> {
    let file = File::open("assets/venues.json").expect("unable to open venues.json");
    let data: serde_json::Value = serde_json::from_reader(file).expect("it is not a json file");

    let mut wtr = Writer::from_path("assets/venues.csv")?;
    wtr.write_record(&["id","title","description","url","street_address","locality","region","postal_code","country","latitude","longitude","email","telephone","events_count"])?;

    let mut count = 0;
    for i in 0..10{
        let id = data[i].get("id").expect("unable to find id").to_string();
        let title = data[i].get("title").expect("unable to find title").to_string();
        let description = data[i].get("description").expect("unable to find description").to_string();
        let url = data[i].get("url").expect("unable to find url").to_string();
        let street_address = data[i].get("street_address").expect("unable to find street_address").to_string();
        let locality = data[i].get("locality").expect("unable to find locality").to_string();
        let region = data[i].get("region").expect("unable to find region").to_string();
        let postal_code = data[i].get("postal_code").expect("unable to find postal_code").to_string();
        let country = data[i].get("country").expect("unable to find country").to_string();
        let latitude = data[i].get("latitude").expect("unable to find latitude").to_string();
        let longitude = data[i].get("longitude").expect("unable to find longitude").to_string();
        let email = data[i].get("email").expect("unable to find email").to_string();
        let telephone = data[i].get("telephone").expect("unable to find telephone").to_string();
        let events_count = data[i].get("events_count").expect("unable to find events_count").to_string();
        // println!("{}", description);
        wtr.write_record(&[id,title,description,url,street_address,locality,region,postal_code,country,
        latitude,longitude,email,telephone,events_count])?;
        count += 1;
    }
    wtr.flush()?;
    Ok(format!("\nSuccessfully saved {} events to assets/venues.csv", count))
}

fn save_to_csv() {
    let status = parse_date();
    match status {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("error running example: {}", e);
            process::exit(1);
        }
    }
}

fn mongodb_driver() -> mongodb::Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My app".to_string());
    Client::with_options(client_options).unwrap()
}

fn db() -> mongodb::Database {
    let client = mongodb_driver();
    client.database("foo")
}

fn coll() -> mongodb::Collection {
    db().collection("venues")
}

fn read_file(file: File) -> csv::Reader<std::io::BufReader<std::fs::File>> {
    csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()))
}

fn insert(file: File) -> Result<String, Box<dyn Error>> {
    let coll = coll();
    coll.delete_many(doc! {}, None)
        .expect("Failed to delete documents.");

    let mut count = 0;
    let mut rdr = read_file(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        coll.insert_one(doc! { "id": record.id, "title": record.title, "description": record.description, "url": record.url, "street_address": record.street_address, "locality": record.locality,"region": record.region, "postal_code": record.postal_code, "country": record.country, "latitude": record.latitude, "longitude": record.longitude, "email": record.email, "telephone": record.telephone, "events_count": record.events_count }, None).unwrap();
        count += 1;
    }
    Ok(format!("\nSuccessfully saved {} venues to MongoDB", count))
}

fn import_to_mongodb() {
    let status = insert(File::open("assets/venues.csv").expect("Failed to read csv."));
    match status {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("error running example: {}", e);
            process::exit(1);
        }
    }
}

fn display() {
    let coll = coll();
    let cursor = coll.find(None, None).unwrap();
    for doc in cursor {
        println!("{}", doc.unwrap());
    }
}

fn search() {
    print!("\nPlease enter: ");
    let find_input = user_input();
    let find = find_input.trim();
    // print!("{}", find);
    let coll = coll();
    let filter = doc! { "summary": find };
    // let find_options = FindOptions::builder()
    //     .sort(doc! { "State": 1 })
    //     .build();
    let cursor = coll.find(filter, None).unwrap();

    for result in cursor {
        match result {
            Ok(document) => println!("Document: {:?}", document),
            Err(e) => println!("Error! {:?}", e),
        }
    }
}

fn user_input() -> std::string::String {
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}

pub fn menu() {
    loop {
        println!("\n1. Scraping all venues data");
        println!("2. Save first 20 venues data to CSV");
        println!("3. Import CSV to MongoDB");
        println!("4. Read events from MongoDB");
        println!("5. Search events from MongoDB");
        println!("0. Back");

        // use the > as the prompt
        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().unwrap();

        match &*command {
            "1" => scraping_event(),
            "2" => save_to_csv(),
            "3" => import_to_mongodb(),
            "4" => display(),
            "5" => search(),
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

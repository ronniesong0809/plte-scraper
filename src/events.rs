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
    start_time: String,
    end_time: String,
    venue_id: String,
    venue_details: String,
}

#[tokio::main]
async fn scraping() -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://calagator.org/events.json").await?;
    let status = response.status();
    let body = response.text().await?;
    // println!("{:?}", status);
    // println!("{:?}", body);
    let mut file = File::create("assets/events.json")?;
    file.write_all(body.as_bytes())?;
    file.sync_all()?;
    Ok(status)
}

fn scraping_event() {
    let html = scraping();
    match html {
        Ok(v) => {
            println!("\nResponse code: {}", v);
            println!("Successfully scraped events to assets/events.json");
        }
        Err(e) => println!("error scraping events: {}", e),
    }
    process::exit(1);
}

#[allow(bare_trait_objects)]
fn parse_date() -> Result<String, Box<Error>> {
    let file = File::open("assets/events.json").expect("unable to open events.json");
    let data: serde_json::Value = serde_json::from_reader(file).expect("it is not a json file");

    let mut wtr = Writer::from_path("assets/events.csv")?;
    wtr.write_record(&["id","title","description","url","start_time","end_time","venue_id","venue_details"])?;

    let mut count = 0;
    for i in 0..100{
        let id = data[i].get("id").expect("unable to find id").to_string();
        let title = data[i].get("title").expect("unable to find title").to_string();
        let description = data[i].get("description").expect("unable to find description").to_string();
        let url = data[i].get("url").expect("unable to find url").to_string();
        let start_time = data[i].get("start_time").expect("unable to find start_time").to_string();
        let end_time = data[i].get("end_time").expect("unable to find end_time").to_string();
        let venue_id = data[i].get("venue_id").expect("unable to find venue_id").to_string();
        let venue_details = data[i].get("venue_details").expect("unable to find venue_details").to_string();
        // println!("{}", description);
        wtr.write_record(&[id,title,description,url,start_time,end_time,venue_id,venue_details])?;
        count += 1;
    }
    wtr.flush()?;
    Ok(format!("\nSuccessfully saved {} events to assets/events.csv", count))
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
    db().collection("events")
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
        coll.insert_one(doc! { "id": record.id, "title": record.title, "description": record.description, "url": record.url, "start_time": record.start_time, "end_time": record.end_time, "venue_id": record.venue_id, "venue_details": record.venue_details }, None).unwrap();
        count += 1;
    }
    Ok(format!("\nSuccessfully saved {} events to MongoDB", count))
}

fn import_to_mongodb() {
    let status = insert(File::open("assets/events.csv").expect("Failed to read csv."));
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
        println!("\n1. Scraping all events data");
        println!("2. Save first 100 events data to CSV");
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

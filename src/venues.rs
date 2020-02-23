extern crate csv;
extern crate mongodb;
extern crate select;
use crate::common::{coll, display, read_file, scraping, search, user_input};
use bson::{bson, doc};
use csv::Writer;
use serde::Deserialize;
use std::{error::Error, fs::File, process};

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

fn scraping_event() {
    let html = scraping("https://calagator.org/venues.json", "assets/venues.json");
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
    for i in 0..100{
        let id = data[i].get("id").expect("unable to find id").to_string().replace("\"", "");
        let title = data[i].get("title").expect("unable to find title").to_string().replace("\"", "");
        let description = data[i].get("description").expect("unable to find description").to_string().replace("\"", "");
        let url = data[i].get("url").expect("unable to find url").to_string().replace("\"", "");
        let street_address = data[i].get("street_address").expect("unable to find street_address").to_string().replace("\"", "");
        let locality = data[i].get("locality").expect("unable to find locality").to_string().replace("\"", "");
        let region = data[i].get("region").expect("unable to find region").to_string().replace("\"", "");
        let postal_code = data[i].get("postal_code").expect("unable to find postal_code").to_string().replace("\"", "");
        let country = data[i].get("country").expect("unable to find country").to_string().replace("\"", "");
        let latitude = data[i].get("latitude").expect("unable to find latitude").to_string().replace("\"", "");
        let longitude = data[i].get("longitude").expect("unable to find longitude").to_string().replace("\"", "");
        let email = data[i].get("email").expect("unable to find email").to_string().replace("\"", "");
        let telephone = data[i].get("telephone").expect("unable to find telephone").to_string().replace("\"", "");
        let events_count = data[i].get("events_count").expect("unable to find events_count").to_string().replace("\"", "");
        println!("\n       title: {}", title);
        println!(" description: {}", description);
        println!("         url: {}", url);
        println!("     address: {}, {}, {} {}, {}", street_address, locality, region, postal_code, country);
        println!("    lat long: {} - {}", latitude, longitude);
        println!("       email: {}", email);
        println!("   telephone: {}", telephone);
        println!("events_count: {}", events_count);
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

fn insert(file: File) -> Result<String, Box<dyn Error>> {
    let coll = coll("venues");
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
            println!("error insert venues: {}", e);
            process::exit(1);
        }
    }
}

pub fn menu() {
    println!("\n\n---- Venues Menu ----");
    loop {
        println!("\n1. Scraping all venues data");
        println!("2. Save first 100 venues data to CSV");
        println!("3. Import CSV to MongoDB");
        println!("4. Read events from MongoDB");
        println!("5. Search events from MongoDB");
        println!("0. Back");

        // use the > as the prompt
        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().expect("unexpected input");

        match &*command {
            "1" => scraping_event(),
            "2" => save_to_csv(),
            "3" => import_to_mongodb(),
            "4" => display("venues"),
            "5" => search("venues", "title"),
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

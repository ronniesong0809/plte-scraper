extern crate csv;
extern crate mongodb;
extern crate select;
use bson::{bson, doc};
use csv::Writer;
use serde::Deserialize;
use std::{ error::Error, fs::File, process };
use crate::common::{user_input, scraping, coll, read_file, display, search};

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

fn scraping_event() {
    let html = scraping("https://calagator.org/events.json", "assets/events.json");
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
        let id = data[i].get("id").expect("unable to find id").to_string().replace("\"", "");
        let title = data[i].get("title").expect("unable to find title").to_string().replace("\"", "");
        let description = data[i].get("description").expect("unable to find description").to_string().replace("\"", "");
        let url = data[i].get("url").expect("unable to find url").to_string().replace("\"", "");
        let start_time = data[i].get("start_time").expect("unable to find start_time").to_string().replace("\"", "");
        let end_time = data[i].get("end_time").expect("unable to find end_time").to_string().replace("\"", "");
        let venue_id = data[i].get("venue_id").expect("unable to find venue_id").to_string().replace("\"", "");
        let venue_details = data[i].get("venue_details").expect("unable to find venue_details").to_string().replace("\"", "");
        println!("\n        title: {}", title);
        println!("  description: {}", description);
        println!("          url: {}", url);
        println!("         time: {} - {}", start_time, end_time);
        println!("     venue_id: {}", venue_id);
        println!("venue_details: {}", venue_details);
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

fn insert(file: File) -> Result<String, Box<dyn Error>> {
    let coll = coll("events");
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

pub fn menu() {
    println!("\n\n---- Events Menu ----");
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
            "4" => display("events"),
            "5" => search("events", "title"),
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

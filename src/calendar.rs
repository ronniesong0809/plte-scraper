extern crate csv;
extern crate mongodb;
extern crate select;
use crate::common::{coll, display, read_file, scraping, search, user_input};
use bson::{bson, doc};
use csv::Writer;
use select::{document::Document, predicate::Class};
use serde::Deserialize;
use std::{error::Error, fs::File, process};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Record {
    summary: String,
    location: String,
    days_of_week: String,
    month: String,
    day: String,
    year: String,
    start: String,
    end: String,
}

fn scraping_event() {
    let html = scraping("https://calagator.org", "assets/calendar.html");
    match html {
        Ok(v) => {
            println!("\nResponse code: {}", v);
            println!("Successfully scraped events to assets/calendar.html");
        }
        Err(e) => println!("error scraping events: {}", e),
    }
    process::exit(1);
}

#[allow(bare_trait_objects)]
pub fn parse_date() -> Result<String, Box<Error>> {
    let mut wtr = Writer::from_path("assets/calendar.csv")?;
    wtr.write_record(&["summary","location","days_of_week","month","day","year","start","end",])?;

    let mut count = 0;
    let document = Document::from(include_str!("../assets/calendar.html"));
    for node in document.find(Class("vevent")) {
        let summary = node.find(Class("summary")).next().unwrap().text();
        let location = node.find(Class("location")).next().unwrap().text();
        let days_of_week = node.find(Class("day_of_week")).next().unwrap().text();
        let temp_date = node.find(Class("list_date")).next().unwrap().last_child().unwrap().text();
        let mut date = temp_date.split_ascii_whitespace();
        let month = date.next().unwrap();
        let day = date.next().unwrap();
        let year = date.next().unwrap();
        let start = node.find(Class("dtstart")).next().unwrap().text();
        let end = node.find(Class("dtend")).next().unwrap().text();
        println!("\n     summary: {}", summary);
        println!("    location: {}", location);
        println!("days_of_week: {}", days_of_week);
        println!("        date: {} {} {}", month, day, year);
        println!("        time: {} - {}", start, end);
        wtr.write_record(&[summary,location,days_of_week,month.to_string(),day.to_string(),year.to_string(),start,end,])?;
        count += 1;
    }
    wtr.flush()?;
    Ok(format!("\nSuccessfully saved {} events to assets/calendar.csv", count))
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
    let coll = coll("calendar");
    coll.delete_many(doc! {}, None)
        .expect("Failed to delete documents.");

    let mut count = 0;
    let mut rdr = read_file(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        coll.insert_one(doc! { "summary": record.summary, "location": record.location, "days_of_week": record.days_of_week, "month": record.month, "day": record.day, "year": record.year, "start": record.start, "end": record.end }, None).unwrap();
        count += 1;
    }
    Ok(format!("\nSuccessfully saved {} events to MongoDB", count))
}

fn import_to_mongodb() {
    let status = insert(File::open("assets/calendar.csv").expect("Failed to read csv."));
    match status {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("error running example: {}", e);
            process::exit(1);
        }
    }
}

pub fn menu() {
    println!("\n\n---- Home Page Calendar Menu ----");
    loop {
        println!("\n1. Scraping home page events data");
        println!("2. Save events data to CSV");
        println!("3. Import CSV to MongoDB");
        println!("4. Read events from MongoDB");
        println!("5. Search events from MongoDB");
        println!("0. Exit");

        // use the > as the prompt
        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().unwrap();

        match &*command {
            "1" => scraping_event(),
            "2" => save_to_csv(),
            "3" => import_to_mongodb(),
            "4" => display("calendar"),
            "5" => search("calendar", "summary"),
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

extern crate csv;
extern crate mongodb;
extern crate select;
use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
// use mongodb::options::FindOptions;
use csv::Writer;
use select::document::Document;
#[allow(unused_imports)]
use select::predicate::{Attr, Class, Name, Predicate};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::process;

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

#[allow(bare_trait_objects)]
pub fn parse_date() -> Result<String, Box<Error>> {
    let document = Document::from(include_str!("../assets/Calendar.html"));
    let mut wtr = Writer::from_path("assets/Calendar.csv")?;
    wtr.write_record(&[
        "summary",
        "location",
        "days_of_week",
        "month",
        "day",
        "year",
        "start",
        "end",
    ])?;

    let mut count = 0;
    for node in document.find(Class("vevent")) {
        let summary = node.find(Class("summary")).next().unwrap().text();
        let location = node.find(Class("location")).next().unwrap().text();
        let days_of_week = node.find(Class("day_of_week")).next().unwrap().text();
        let temp_date = node
            .find(Class("list_date"))
            .next()
            .unwrap()
            .last_child()
            .unwrap()
            .text();
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
        wtr.write_record(&[
            summary,
            location,
            days_of_week,
            month.to_string(),
            day.to_string(),
            year.to_string(),
            start,
            end,
        ])?;
        count += 1;
    }
    wtr.flush()?;
    Ok(format!(
        "\nSuccessfully saved {} events to assets/Calendar.csv",
        count
    ))
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
    db().collection("bar")
}

fn open_file() {
    let status = insert(File::open("assets/Calendar.csv").expect("Failed to read csv."));
    match status {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("error running example: {}", e);
            process::exit(1);
        }
    }
}

fn insert(file: File) -> Result<&'static str, Box<dyn Error>> {
    let coll = coll();
    coll.delete_many(doc! {}, None)
        .expect("Failed to delete documents.");
    let mut rdr = read_file(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        coll.insert_one(doc! { "summary": record.summary, "location": record.location, "days_of_week": record.days_of_week, "month": record.month, "day": record.day, "year": record.year, "start": record.start, "end": record.end }, None).unwrap();
    }
    Ok("\nSuccessfully saved to MongoDB")
}

fn read_file(file: File) -> csv::Reader<std::io::BufReader<std::fs::File>> {
    csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()))
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

fn main() {
    loop {
        // use the > as the prompt
        println!("\n1. Save to CSV");
        println!("2. Insert to MongoDB");
        println!("3. Display All");
        println!("4. Search");
        println!("0. Exit");

        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().unwrap();

        match &*command {
            "1" => save_to_csv(),
            "2" => open_file(),
            "3" => display(),
            "4" => search(),
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

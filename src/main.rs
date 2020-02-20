extern crate csv;
extern crate mongodb;
use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
// use mongodb::options::FindOptions;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::process;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Record {
    City: String,
    State: String,
    Population: String,
    Latitude: String,
    Longitude: String,
}

fn mongodb_driver() -> mongodb::Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My app".to_string());
    let client = Client::with_options(client_options).unwrap();
    return client;
}

fn db() -> mongodb::Database {
    let client = mongodb_driver();
    let db = client.database("foo");
    return db;
}

fn coll() -> mongodb::Collection {
    let coll = db().collection("bar");
    return coll;
}

fn open_file() {
    if let Err(err) = insert(File::open("csv_test.csv").expect("Failed to read csv.")) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn insert(file: File) -> Result<(), Box<dyn Error>> {
    let coll = coll();
    coll.delete_many(doc! {}, None)
        .ok()
        .expect("Failed to delete documents.");
    let mut rdr = read_file(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        coll.insert_one(doc! { "City": record.City, "State": record.State, "Population": record.Population, "Latitude": record.Latitude, "Longitude": record.Longitude }, None).unwrap();
    }
    Ok(())
}

fn read_file(file: File) -> csv::Reader<std::io::BufReader<std::fs::File>> {
    return csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()));
}

fn display() {
    let coll = coll();
    let cursor = coll.find(None, None).unwrap();
    for doc in cursor {
        println!("{}", doc.unwrap());
    }
}

fn search(find: &str) {
    let coll = coll();
    let filter = doc! { "City": find };
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

fn user_input () -> std::string::String{
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn main() {
    loop {
        // use the > as the prompt
        println!("\n1. Insert to MongoDB");
        println!("2. Display All");
        println!("3. Search");
        println!("0. Exit");

        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().unwrap();

        match &*command {
            "1" => open_file(),
            "2" => display(),
            "3" => {
                print!("\nPlease enter: ");
                let find_input = user_input();
                let find = &find_input;
                search(find);
            },
            "0" => return,
            "q" => return,
            "quit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}

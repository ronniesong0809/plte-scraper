extern crate mongodb;
extern crate csv;
use std::error::Error;
use std::process;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use mongodb::{bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient, db::ThreadedDatabase, common::{ReadPreference, ReadMode}};
// use csv::Reader;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Record {
    City: String,
    State: String,
    Population: String,
    Latitude: String,
    Longitude: String,
}

fn mongodb_driver(file: File) -> Result<(), Box<dyn Error>>{
    let mut options = ClientOptions::new();
    options.read_preference = Some(ReadPreference::new(ReadMode::PrimaryPreferred, None));

    let client = Client::with_uri_and_options("mongodb://localhost:27017/foo", options).unwrap();
    let coll = client.db("foo").collection("bar");
    coll.drop().unwrap();

    let mut rdr = csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()));
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Record = result?;
        coll.insert_one(doc! { "City": record.City, "State": record.State, "Population": record.Population, "Latitude": record.Latitude, "Longitude": record.Longitude }, None).unwrap();
    }


    for doc in coll.find(None, None).unwrap() {
        println!("{}", doc.unwrap());
    }
    Ok(())
}

fn main() {
    if let Err(err) = mongodb_driver(File::open("csv_test.csv").unwrap()) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
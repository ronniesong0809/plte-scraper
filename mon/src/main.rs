extern crate mongodb;
extern crate csv;
use mongodb::{bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient, db::ThreadedDatabase, common::{ReadPreference, ReadMode}};
use std::error::Error;
use csv::Reader;

fn main() {
    let mut options = ClientOptions::new();
    options.read_preference = Some(ReadPreference::new(ReadMode::PrimaryPreferred, None));

    let client = Client::with_uri_and_options("mongodb://localhost:27017/foo", options).unwrap();
    let coll = client.db("foo").collection("bar");
    coll.drop().unwrap();

    coll.insert_one(doc! { "testName": "BOX Zhang" }, None).unwrap();

    for doc in coll.find(None, None).unwrap() {
        println!("{}", doc.unwrap());
    }
}


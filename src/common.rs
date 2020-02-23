extern crate mongodb;
use bson::{bson, doc};
use mongodb::{ options::ClientOptions, Client };
use std::{ fs::File, io::{BufReader, stdin, stdout, Write} };

#[tokio::main]
pub async fn scraping(url: &str, dist: &str) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let status = response.status();
    let body = response.text().await?;
    // println!("{:?}", status);
    println!("{:?}", body);
    let mut file = File::create(dist)?;
    file.write_all(body.as_bytes())?;
    file.sync_all()?;
    Ok(status)
}

fn mongodb_driver() -> mongodb::Client {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My app".to_string());
    Client::with_options(client_options).unwrap()
}

fn db() -> mongodb::Database {
    let client = mongodb_driver();
    client.database("calagator")
}

pub fn coll(coll_name: &str) -> mongodb::Collection {
    db().collection(coll_name)
}

pub fn read_file(file: File) -> csv::Reader<std::io::BufReader<std::fs::File>> {
    csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()))
}

pub fn display(coll_name: &str) {
    let coll = coll(coll_name);
    let cursor = coll.find(None, None).unwrap();
    for doc in cursor {
        println!("{}", doc.unwrap());
    }
}

pub fn search(coll_name: &str, field: &str) {
    print!("\nPlease enter: ");
    let find_input = user_input();
    let find = find_input.trim();
    // print!("{}", find);
    let coll = coll(coll_name);
    let filter = doc! { field: find };
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

pub fn user_input() -> std::string::String {
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    // print!("{:?}", input);
    if input=="\n"{
        input = "0".to_string();
    }
    input
}
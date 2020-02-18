
use std::error::Error;
use std::process;
use std::fs::File;
use std::io::BufReader;

fn example(file: File) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(BufReader::new(file.try_clone().unwrap()));
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example(File::open("csv_test.csv").unwrap()) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
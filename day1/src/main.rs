extern crate csv;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./data/data.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut oldRecord = 0;
    let mut currentRecord = 0;
    let mut result = 0;

    for record in rdr.records() {
        let thisRecord = record?;
        for newRecord in &thisRecord {
            println!("{:?}", newRecord);
            let currentRecord = newRecord.parse::<i32>().unwrap();
            if oldRecord < currentRecord {
                result += 1;
            }
            oldRecord = currentRecord
        }

    }
    println!("{:?}", result);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

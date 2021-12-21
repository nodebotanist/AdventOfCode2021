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

    let mut result = 0;
    let mut inner_result = 0;
    let mut outer_result = 0;
    let mut records:[i32;3] = [-1, -1, -1];

    for record in rdr.records() {
        let thisRecord = record?;
        let mut continue_sums = true;
        for newRecord in &thisRecord {
            println!("{:?}", newRecord);
            let currentRecord = newRecord.parse::<i32>().unwrap();
            if records[0] == -1 {
                records[0] = currentRecord;
                continue_sums = false;
            } else if records[1] == -1 {
                continue_sums = false;
                records[1] = currentRecord;
            } else if records[2] == -1 {
                continue_sums = false;
                records[2] = currentRecord;
                inner_result = records[0] + records[1] + records[2];
            }
            if continue_sums {
                records[0] = records[1];
                records[1] = records[2];
                records[2] = currentRecord;
                outer_result = records[0] + records[1] + records[2];

                if outer_result > inner_result {
                    result += 1;
                }

                inner_result = outer_result;
            }
            continue_sums = true;
            println!("{}, {}, {}", records[0], records[1], records[2]);
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

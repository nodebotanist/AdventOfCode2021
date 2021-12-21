use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    one();
}

fn one(){
    let mut horizontal = 0;
    let mut vertical = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines{
            if let Ok(ref ip) = &line {
                let split = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
                let instruction = split[0];
                let val: i32 = split[1].parse().unwrap();

                println!("{}, {}", instruction, val);
                match instruction {
                    "up" => vertical = vertical - val,
                    "down" => vertical = vertical + val,
                    "forward" => horizontal = horizontal + val,
                    _ => ()
                };
            }
        }
    }

    println!("Total values: {}, {}, {}", horizontal, vertical, horizontal * vertical);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
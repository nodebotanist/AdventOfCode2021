use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    one();
}

fn one(){
    let mut byte_count = [0,0,0,0,0,0,0,0,0,0,0,0];
    let mut total_count = 0;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines{
            if let Ok(ref ip) = &line {
                let val = isize::from_str_radix(&line.unwrap(), 2).unwrap();

                println!("{}", val);

                for index in 0..12 {
                    if val & (0b1 << index) == (0b1 << index) {
                        println!("bit {} matched", index);
                        byte_count[index] += 1;
                    }
                }

                total_count += 1;
            }
        }
        
        for index in 0..12 {
            if (byte_count[index] * 2) > total_count {
                gamma |= 0b1 << index;
            } else {
                epsilon |= 0b1 << index;
            }
        }

        println!("Gamma: {}", gamma);
        println!("Epsilon: {}", epsilon);
        println!("Answer: {}", gamma * epsilon);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
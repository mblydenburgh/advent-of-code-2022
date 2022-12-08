use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(file_name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut biggest: u32 = 0;
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("data.txt") {
        for line in lines {
            println!("The current biggest is {}", biggest);

            if let Ok(value) = line {
            println!("Current value: {}", value);
                if value == "" {
                    sum = 0;
                    continue;
                }
                match value.parse::<u32>() {
                    Ok(val) => sum += val,
                    Err(err) => panic!("Error: {}", err)
                }
            }
            println!("Current sum: {}", sum);
            if sum > biggest {
                biggest = sum;
            }
        }
    }
}

use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(file_name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

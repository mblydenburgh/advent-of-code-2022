use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(file_name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum: u32 = 0;
    let mut elf_totals: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("data.txt") {
        for line in lines {

            if let Ok(value) = line {
                if value == "" {
                    sum = 0;
                    continue;
                }
                match value.parse::<u32>() {
                    Ok(val) => sum += val,
                    Err(err) => panic!("Error: {}", err)
                }
            }
            elf_totals.push(sum);

        }
        elf_totals.sort_by(|a, b| b.cmp(a));
        println!("Elf totals: {:?}", elf_totals);
        let ans: u32 = match elf_totals.iter().max() {
            Some(val) => val.to_owned(),
            None => 0
        };
        println!("Answer 1: {}", ans);

        let mut ans2: u32 = 0;
        for n in 0..3 {
           ans2 = ans2 + elf_totals[n];
        }
       println!("Answer 2: {}", ans2);
    }
}

use crate::days::utils::read_lines;

pub fn day_one() -> String {
    let mut sum: u32 = 0;
    let mut elf_totals: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("day-1-data.txt") {
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
        let ans: u32 = match elf_totals.iter().max() {
            Some(val) => val.to_owned(),
            None => 0
        };

        let mut ans2: u32 = 0;
        for n in 0..3 {
           ans2 = ans2 + elf_totals[n];
        }
       return format!("Answer 1: {}, Answer 2: {}", ans, ans2)
    }
    return "No answer found".to_string()
}

use crate::days::utils::read_lines;

fn check_ranges(range_1: &Vec<u8>, range_2: &Vec<u8>) -> bool {
    let mut encapsulated: bool = false;
    if range_1[0] >= range_2[0] && range_1[1] <= range_2[1] {
        // range_1 encapsulated in range_2
        encapsulated = true
    }
    if range_2[0] >= range_1[0] && range_2[1] <= range_1[1] {
        encapsulated = true
    }

    return encapsulated
}

pub fn day_4() -> String {
    if let Ok(lines) = read_lines("day-4-data.txt") {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let ranges: Vec<&str> = value.split(",").collect();
                let range_1: Vec<u8> = ranges[0].split("-").map(|val| val.parse::<u8>().unwrap()).collect();
                let range_2: Vec<u8> = ranges[1].split("-").map(|val| val.parse::<u8>().unwrap()).collect();
                if check_ranges(&range_1, &range_2) {
                    sum = sum + 1;
                }
            }
        }
        return format!("Answer: {:?}", sum);
    }
    return "No answer found".to_string()
}

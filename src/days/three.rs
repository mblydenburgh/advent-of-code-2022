use crate::days::utils::read_lines;

fn calc_score(dupes: Vec<char>) -> u32 {
    let score_str: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score: u32 = 0;
    for c in dupes {
        let char_score: u32 = match score_str.chars().position(|p| p == c) {
            Some(val) => val as u32 + 1,
            None => panic!("Could not find score")
        };
        score = score + char_score;
    }
    score
}

pub fn day_3() -> String {
    if let Ok(lines) = read_lines("day-3-data.txt") {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let mut duplicates: Vec<char> = Vec::new();
                let line_len: usize = value.len();
                let line_chars: Vec<char> = value.chars().collect();
                let (left_half, right_half) = line_chars.split_at(line_len / 2);
                let left_half_vec: Vec<char> = left_half.to_vec();
                let right_half_vec: Vec<char> = right_half.to_vec();
                for l in left_half_vec {
                    if right_half_vec.contains(&l) {
                        if !duplicates.contains(&l) {
                            duplicates.push(l);
                        }
                    };
                }
                sum = sum + calc_score(duplicates);
            }
        }
        return format!("Answer: {:?}", sum);
    }
    return "No answer found".to_string()

}
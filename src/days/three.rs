use std::collections::HashMap;

use crate::days::utils::read_lines;

fn generate_score_map() -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    map.insert("a".chars().next().unwrap(), 1);
    map.insert("b".chars().next().unwrap(), 2);
    map.insert("c".chars().next().unwrap(), 3);
    map.insert("d".chars().next().unwrap(), 4);
    map.insert("e".chars().next().unwrap(), 5);
    map.insert("f".chars().next().unwrap(), 6);
    map.insert("g".chars().next().unwrap(), 7);
    map.insert("h".chars().next().unwrap(), 8);
    map.insert("i".chars().next().unwrap(), 9);
    map.insert("j".chars().next().unwrap(), 10);
    map.insert("k".chars().next().unwrap(), 11);
    map.insert("l".chars().next().unwrap(), 12);
    map.insert("m".chars().next().unwrap(), 13);
    map.insert("n".chars().next().unwrap(), 14);
    map.insert("o".chars().next().unwrap(), 15);
    map.insert("p".chars().next().unwrap(), 16);
    map.insert("q".chars().next().unwrap(), 17);
    map.insert("r".chars().next().unwrap(), 18);
    map.insert("s".chars().next().unwrap(), 19);
    map.insert("t".chars().next().unwrap(), 20);
    map.insert("u".chars().next().unwrap(), 21);
    map.insert("v".chars().next().unwrap(), 22);
    map.insert("w".chars().next().unwrap(), 23);
    map.insert("x".chars().next().unwrap(), 24);
    map.insert("y".chars().next().unwrap(), 25);
    map.insert("z".chars().next().unwrap(), 26);
    map.insert("A".chars().next().unwrap(), 27);
    map.insert("B".chars().next().unwrap(), 28);
    map.insert("C".chars().next().unwrap(), 29);
    map.insert("D".chars().next().unwrap(), 30);
    map.insert("E".chars().next().unwrap(), 31);
    map.insert("F".chars().next().unwrap(), 32);
    map.insert("G".chars().next().unwrap(), 33);
    map.insert("H".chars().next().unwrap(), 34);
    map.insert("I".chars().next().unwrap(), 35);
    map.insert("J".chars().next().unwrap(), 36);
    map.insert("K".chars().next().unwrap(), 37);
    map.insert("L".chars().next().unwrap(), 38);
    map.insert("M".chars().next().unwrap(), 39);
    map.insert("N".chars().next().unwrap(), 40);
    map.insert("O".chars().next().unwrap(), 41);
    map.insert("P".chars().next().unwrap(), 42);
    map.insert("Q".chars().next().unwrap(), 43);
    map.insert("R".chars().next().unwrap(), 44);
    map.insert("S".chars().next().unwrap(), 45);
    map.insert("T".chars().next().unwrap(), 46);
    map.insert("U".chars().next().unwrap(), 47);
    map.insert("V".chars().next().unwrap(), 48);
    map.insert("W".chars().next().unwrap(), 49);
    map.insert("X".chars().next().unwrap(), 50);
    map.insert("Y".chars().next().unwrap(), 51);
    map.insert("Z".chars().next().unwrap(), 52);

    map
}

fn calc_score(dupes: Vec<char>) -> u32 {
    let score_map: HashMap<char, u32> = generate_score_map();
    let mut score: u32 = 0;
    for c in dupes {
        let char_score: u32 = match score_map.get(&c) {
            Some(val) => val.to_owned(),
            None => panic!("Could not get score for char")
        };
        score = score + char_score;
    }
    score
}

pub fn day_3() -> String {
    if let Ok(lines) = read_lines("day-3-data.txt") {
        let mut duplicates: Vec<char> = Vec::new();
        let mut sum: u32 = 0;
        for line in lines {

            if let Ok(value) = line {
                let line_len: usize = value.len();
                let line_chars: Vec<char> = value.chars().collect();
                let (left_half, right_half) = line_chars.split_at(line_len / 2);

                for l in left_half {
                    for r in right_half {
                        if l == r {
                            let val = l.clone();
                            duplicates.push(val);
                        }
                    }
                }
                sum = sum + calc_score(duplicates);
            }
        }
       return format!("Answer: {:?}", sum);
    }
    return "No answer found".to_string()

}

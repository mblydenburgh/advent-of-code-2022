use crate::days::utils::read_lines;

enum Outcome {
    Win,
    Lose,
    Draw
}

enum Choice {
    Rock,
    Paper,
    Scissor
}

fn calc_round_score(choice: &Choice, outcome: Outcome) -> u32 {
    let choice_score: u32 = match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissor => 3
    };
    let outcome_score: u32 = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0
    };
    return choice_score + outcome_score
}

fn determine_round(player: &Choice, elf: Choice) -> Outcome {
    match player {
        Choice::Rock => {
            match elf {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Lose,
                Choice::Scissor => Outcome::Win
            }
        },
        Choice::Paper => {
            return match elf {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Draw,
                Choice::Scissor => Outcome::Lose
            }
        },
        Choice::Scissor => {
            return match elf {
                Choice::Rock => Outcome::Lose,
                Choice::Paper => Outcome::Win,
                Choice::Scissor => Outcome::Draw
            }
        }
    }
}

fn determine_choice(input: &str) -> Choice {
    match input {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissor,
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissor,
        &_ => panic!("Cannot parse choice")
    }
}

fn determine_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        &_ => panic!("Could not parse outcome")
    }
}

fn determine_player_choice(elf_choice: &Choice, outcome: &Outcome) -> Choice {
    match elf_choice {
        Choice::Rock => {
            match outcome {
                Outcome::Win => Choice::Paper,
                Outcome::Draw => Choice::Rock,
                Outcome::Lose => Choice::Scissor
            }
        },
        Choice::Paper => {
            match outcome {
                Outcome::Win => Choice::Scissor,
                Outcome::Draw => Choice::Paper,
                Outcome::Lose => Choice::Rock
            }
        },
        Choice::Scissor => {
            match outcome {
                Outcome::Win => Choice::Rock,
                Outcome::Draw => Choice::Scissor,
                Outcome::Lose => Choice::Paper
            }
        }
    }
}

pub fn day_2() -> String {
    let mut scores: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("day-2-data.txt") {
        for line in lines {
            if let Ok(value) = line {
                let raw_choices: Vec<&str> = value.split_whitespace().collect();
                let player_choice: Choice = determine_choice(raw_choices[1]);
                let elf_choice: Choice = determine_choice(raw_choices[0]);
                let outcome: Outcome = determine_round(&player_choice, elf_choice);
                let score: u32 = calc_round_score(&player_choice, outcome);
                scores.push(score);
            }
        }
        let total: u32 = scores.iter().sum();
        return format!("Total score: {}", total)
    }
    return "No data found".to_string()
}

pub fn day_2_pt2() -> String {
    let mut scores: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("day-2-data.txt") {
        for line in lines {
            if let Ok(value) = line {
                let raw_values: Vec<&str> = value.split_whitespace().collect();
                let elf_choice: Choice = determine_choice(raw_values[0]);
                let outcome: Outcome = determine_outcome(raw_values[1]);
                let player_choice: Choice = determine_player_choice(&elf_choice, &outcome);
                let score: u32 = calc_round_score(&player_choice, outcome);
                scores.push(score);
            }
        }
        let total: u32 = scores.iter().sum();
        return format!("Total score of part 2: {}", total)
    }
    return "No data found".to_string()
}

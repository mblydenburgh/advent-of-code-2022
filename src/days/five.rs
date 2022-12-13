use regex::Regex;
use crate::days::utils::read_lines;

struct Step {
    from: u32,
    to: u32,
    amount: u32
}

fn generate_stacks(line: &String) -> Vec<Vec<char>> {
    let length = (line.len() + 1) / 4;
    let mut stack = Vec::with_capacity(length);
    for _i in 0..length {
        stack.push(Vec::new())
    }
    stack
}

fn build_ans(stacks: Vec<Vec<char>>) -> String {
    let mut ans: String = String::new();
    for stack in stacks {
        ans.push_str(&stack.get(usize::try_from(stack.len()).unwrap() - 1).unwrap().to_string());
    }
    
    ans
}

pub fn day_5() -> String {
    if let Ok(lines) = read_lines("day-5-data.txt") {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let mut steps: Vec<Step> = Vec::new();
        let stack_regex: Regex = Regex::new(r"^[\[][\sA-Z\[\]]+[\]\s]$").unwrap(); 
        let step_regex: Regex = Regex::new(r"^[\w]+\s(\d+)\s\w+\s(\d+)\s\w+\s(\d+)$").unwrap();
        for line in lines {
            // number of stacks is (length of string + 1) / 4
            // parse each line while keeping track of current index
            // check value for alphabetic
            // assign to col vec based on current line index
            if let Ok(value) = line {
                if stacks.len() == 0 {
                    stacks = generate_stacks(&value); 
                }
                if stack_regex.is_match(&value) {
                    let line_chars: Vec<char> = value.chars().collect();
                    for (i, val) in line_chars.into_iter().enumerate() {
                        if val.is_alphabetic() {
                            // determine what stack to put the value in based on index
                            let stack_num: usize = match i {
                                1 => 0,
                                _ => (i - 1) / 4
                            };
                        stacks[stack_num].push(val);
                    }
                }
                } else if step_regex.is_match(&value) {
                    let regex_cap = match step_regex.captures(&value) {
                        Some(val) => val,
                        None => panic!("Could parse step")
                    };
                    let step = Step {
                        from: regex_cap.get(2).map(|v| v.as_str()).unwrap().parse::<u32>().unwrap(),
                        to: regex_cap.get(3).map(|v| v.as_str()).unwrap().parse::<u32>().unwrap(),
                        amount: regex_cap.get(1).map(|v| v.as_str()).unwrap().parse::<u32>().unwrap()                    
                    };
                    steps.push(step);
                } else {
                    continue;
                }
            }
        }

        // Reverse order in all initial stacks so the "top" items can be popped off and pushed on
        let mut rev_stacks: Vec<Vec<char>> = Vec::new();
        for stack in stacks {
            let rev_stack: Vec<char> = stack.into_iter().rev().collect();
            rev_stacks.push(rev_stack);
        }

        for step in steps {
            // TODO - find how to get a non-reference from rev_stack so popping off occurs on actual vec and not a &Vec
            let source_stack_num: usize = usize::try_from(step.from).unwrap() - 1;
            let target_stack_num: usize = usize::try_from(step.to).unwrap() -1;
            let mut from_vec: Vec<char> = rev_stacks.get(source_stack_num).unwrap().to_owned();
            let final_len = from_vec.len() - usize::try_from(step.amount).unwrap();
            let taken: Vec<char> = from_vec.split_off(final_len).into_iter().rev().collect();
            rev_stacks[source_stack_num] = from_vec;
            for val in taken {
                rev_stacks[target_stack_num].push(val);    
            }
        }

        let ans = build_ans(rev_stacks);
        return format!("Answer: {:?}", ans);
    }
    return "No answer found".to_string()

}

use crate::days::utils::read_lines;

fn generate_stacks(line: String) -> Vec<Vec<char>> {
    let length = (line.len() + 1) / 4;
    let mut stack = Vec::with_capacity(length);
    for _i in 0..length {
        stack.push(Vec::new())
    }
    stack
}

pub fn day_5() -> String {
    if let Ok(mut lines) = read_lines("day-5-data.txt") {
        let mut stacks: Vec<Vec<char>> = generate_stacks(lines.next().unwrap().unwrap());

        for line in lines {
            // number of stacks is (length of string + 1) / 4
            // parse each line while keeping track of current index
            // check value for alphabetic
            // assign to col vec based on current line index
            if let Ok(value) = line {
                if value.len() > 0 {
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
                } else {
                    break;
                }
            }
        }
        return format!("Answer: {:?}", stacks);
    }
    return "No answer found".to_string()

}

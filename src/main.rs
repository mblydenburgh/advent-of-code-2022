mod days;

use self::days::{
    one::day_one,
    utils::read_lines
};

fn main() {
    println!("*** DAY 1***");
    let ans: String = day_one();
    println!("ANSWER: {}", ans);
}

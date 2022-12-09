mod days;

use self::days::{
    one::day_one,
    two::{
        day_2,
        day_2_pt2
    }
};

fn main() {
    println!("*** DAY 1 ***");
    let ans1: String = day_one();
    println!("ANSWER: {}", ans1);
    println!("*** DAY 2 ***");
    let ans2: String = day_2();
    println!("ANSWER: {}", ans2);
    let ans2_2: String = day_2_pt2();
    println!("ANSWER 2: {}", ans2_2);
}

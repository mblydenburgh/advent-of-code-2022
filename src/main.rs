mod days;

use crate::days::six::day_6;

use self::days::{
    one::day_one,
    two::{
        day_2,
        day_2_pt2
    },
    three::day_3,
    four::day_4,
    five::day_5
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
    println!("*** Day 3 ***");
    let ans3: String = day_3();
    println!("ANSWER 3: {}", ans3);
    println!("*** Day 4 ***");
    let ans4: String = day_4();
    println!("ANSWER 4: {}", ans4);
    println!("*** Day 5 ***");
    let ans5: String = day_5();
    println!("ANSWER 5: {}", ans5);
    println!("*** Day 6 ***");
    let ans6: String = day_6();
    println!("ANSWER 6: {}", ans6);
}

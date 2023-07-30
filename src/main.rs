use std::time::Instant;
use std::fs;
mod day2;
mod day1;
mod day3;


use day3::item_types_day3_2;
fn main() {
    let mut input_file  = fs::read_to_string("src/day3/input.txt").expect("expect a file here");
    let now = Instant::now();
    let result = item_types_day3_2(&mut input_file);
    let now2 = Instant::now();
    let time = now2-now;
    println!("{:?}",time);
    println!("{}",result);
}

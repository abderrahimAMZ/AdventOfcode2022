use std::time::Instant;
use std::fs;
mod day2;
mod day1;
mod day3;
mod day4;


use day4::camp_cleanup;
fn main() {
    let mut input_file  = fs::read_to_string("src/day4/input.txt").expect("expect a file here");
    let now = Instant::now();
    let result = camp_cleanup(&mut input_file);
    let now2 = Instant::now();
    let time = now2-now;
    println!("{:?}",time);
    println!("{}",result);
}

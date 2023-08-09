use std::fs;
use std::time::Instant;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use day4::camp_cleanup;
use day5::manage_crates;
use day6::get_marker;
fn main() {
    let mut input_file = fs::read_to_string("src/day6/input.txt").expect("expect a file here");
    let now = Instant::now();
    let marker = get_marker(&mut input_file);
    let now2 = Instant::now();
    let time = now2 - now;
    println!("{:?}", time);
    println!("this is the marker result : {}", marker);
}

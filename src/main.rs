use std::fs;
use std::time::Instant;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use day4::camp_cleanup;
use day5::manage_crates;
use day6::get_marker;
use day7::create_tree_day7;
use day8::how_many_visible_tree;
fn main() {
    let mut input_file = fs::read_to_string("src/day8/input.txt").expect("expect a file here");
    let now = Instant::now();
    let result = how_many_visible_tree(&mut input_file);
    let now2 = Instant::now();
    let time = now2 - now;
    println!("{:?}", time);
    println!("{:?}", result);
}

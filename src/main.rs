use std::time::Instant;
use std::fs;
mod day2;
mod day1;

use day2::playing_rock_paper_scissor_2;
fn main() {
    let mut input_file  = fs::read_to_string("src/day2/input.txt").expect("expect a file here");
    let now = Instant::now();
    let result = playing_rock_paper_scissor_2(&mut input_file);
    let now2 = Instant::now();
    let time = now2-now;
    println!("{:?}",time);
    println!("{}",result);
}

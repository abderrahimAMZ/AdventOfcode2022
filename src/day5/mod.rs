use crate::day4::ParseAssignmentError;
use std::fs::create_dir;
use std::num::ParseIntError;
use std::str::FromStr;
#[derive(Debug)]
pub struct Command {
    how_many: usize,
    source: usize,
    destination: usize,
}
#[derive(Debug)]
pub enum ParseCommandError {
    MissingQuantity(ParseIntError),
    MissingSource(ParseIntError),
    MissingDestination(ParseIntError),
}
impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let infos: Vec<_> = s.split_whitespace().collect();
        let (Some(&a),Some(&b),Some(&c)) = (infos.get(1),infos.get(3),infos.get(5)) else {panic!("infos missing")};

        Ok(Self {
            how_many: a
                .parse::<usize>()
                .map_err(ParseCommandError::MissingQuantity)?,
            source: b
                .parse::<usize>()
                .map_err(ParseCommandError::MissingSource)?
                - 1,
            destination: c
                .parse::<usize>()
                .map_err(ParseCommandError::MissingDestination)?
                - 1,
        })
    }
}

pub fn manage_crates(input: &str) {
    let mut crates: Vec<Vec<&str>> = vec![
        vec!["Q", "S", "W", "C", "Z", "V", "F", "T"],
        vec!["Q", "R", "B"],
        vec!["B", "Z", "T", "Q", "P", "M", "S"],
        vec!["D", "V", "F", "R", "Q", "H"],
        vec!["J", "G", "L", "D", "B", "S", "T", "P"],
        vec!["W", "R", "T", "Z"],
        vec!["H", "Q", "M", "N", "S", "F", "R", "J"],
        vec!["R", "N", "F", "H", "W"],
        vec!["J", "Z", "T", "Q", "P", "R", "B"],
    ];
    input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .for_each(|Command| {
            let source = &mut crates[Command.source];
            let source_len = source.len();
            let spliting_at = source_len - Command.how_many;
            let need_to_be_moved = source.split_off(spliting_at);
            let destination = &mut crates[Command.destination];
            destination.extend(need_to_be_moved);
        });

    crates
        .iter_mut()
        .for_each(|a_crate| print!("{}", a_crate.pop().unwrap()));
    println!();
}

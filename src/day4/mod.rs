use crate::day4::ParseAssignmentError::{MissingDash, MissingFirst, MissingLast};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Assignment {
    first: i16,
    last: i16,
}
#[derive(Debug)]
pub enum ParseAssignmentError {
    MissingDash,
    MissingFirst(ParseIntError),
    MissingLast(ParseIntError),
}
impl FromStr for Assignment {
    type Err = ParseAssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(MissingDash)?;

        Ok(Self {
            first: a
                .parse::<i16>()
                .map_err(ParseAssignmentError::MissingFirst)?,
            last: b
                .parse::<i16>()
                .map_err(ParseAssignmentError::MissingLast)?,
        })
    }
}
impl Assignment {
    pub fn contains(&self, ass: &Assignment) -> bool {
        self.first <= ass.first && self.last >= ass.last
    }
    pub fn overlaps(&self, ass: &Assignment) -> bool {
        (self.last >= ass.first && self.first <= ass.first)
            || (self.first <= ass.last && self.last >= ass.last)
    }
}
pub fn camp_cleanup(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| {
            let ass1 = a.parse::<Assignment>().unwrap();
            let ass2 = b.parse::<Assignment>().unwrap();
            (ass1, ass2)
        })
        .filter(|(ass1, ass2)| ass1.overlaps(ass2) || ass2.overlaps(ass1))
        .count()
}

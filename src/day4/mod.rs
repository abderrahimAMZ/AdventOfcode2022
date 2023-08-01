use std::convert::TryInto;
use std::num::ParseIntError;
use std::str::FromStr;
#[derive(Debug)]
pub struct assignment {
    first : i16,
    last : i16
}
impl FromStr for assignment {
    type Err = i16;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a,b) = s.split_at(s.find('-').unwrap()) else {
            return Err(-20)
        };
        let (b,c) = b.split_at(1) else {
            return Err(-20)
        };
        Ok( Self {
            first : a.parse::<i16>().unwrap(),
            last : c.parse::<i16>().unwrap()
        })
    }
}
impl assignment {
    pub fn contains(&self, ass : &assignment)-> bool {
        self.first <= ass.first  &&  self.last >= ass.last
    }
}
pub fn camp_cleanup(input :&str)-> usize {
    input
        .lines()
        .map(|line| {
            let index = line.find(',').unwrap();
            let (a,b) = line.split_at(index);
            let (b,c) = b.split_at(1);
            (a,c) } )
        .map(|(a,b)| {
            let ass1 = a.parse::<assignment>();
            let ass2 = b.parse::<assignment>();
            (ass1.unwrap(),ass2.unwrap())
        })
        .filter(|(ass1,ass2)| {
            let is_contained = ass1.contains(ass2) || ass2.contains(ass1);
            is_contained
        })
        .collect::<Vec<_>>()
        .len()
}
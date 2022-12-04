extern crate array_tool;
use array_tool::vec::Intersect;

use std::{fs, str::FromStr, num::ParseIntError};

struct Sequence {
    start: i32,
    stop: i32,
}

impl Sequence {
    fn get_range_inc(&self) -> Vec<i32> {
        (self.start..(self.stop + 1)).collect()
    }

    fn either_fully_envelope(&self, other: &Sequence) -> Option<()> {
        let first_seq = self.get_range_inc();
        let second_seq = other.get_range_inc();

        let intersect = first_seq.intersect(second_seq.to_owned());
        if intersect.len() == first_seq.len() || intersect.len() == second_seq.len() {
            Some(())
        } else {
            None
        }
    }
}

impl FromStr for Sequence {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, stop ) = s.split_once('-').unwrap();

        let start_fromstr = start.parse::<i32>()?;
        let stop_fromstr = stop.parse::<i32>()?;

        Ok(Sequence { start: start_fromstr, stop: stop_fromstr })
    }
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn part1(input: Vec<String>) -> usize {
    let overlaps: Vec<Option<()>> = input
        .iter()
        .map(|f| {
            let seq: Vec<Sequence> = f
                .split(',')
                .map(|s| s.parse::<Sequence>().unwrap())
                .collect();

            let first_seq = seq.first().unwrap().to_owned();
            let second_seq = seq.get(1).unwrap().to_owned();

            first_seq.either_fully_envelope(second_seq)
        })
        .filter(|f| f.is_some())
        .collect();

    overlaps.len()
}

fn part2(input: Vec<String>) -> usize {
    let overlaps: Vec<usize> = input
        .iter()
        .map(|f| {
            let seq: Vec<Sequence> = f
                .split(',')
                .map(|s| s.parse::<Sequence>().unwrap())
                .collect();

            let first_seq = seq.first().unwrap().to_owned().get_range_inc();
            let second_seq = seq.get(1).unwrap().to_owned().get_range_inc();

            first_seq.intersect(second_seq).len()
        })
        .filter(|f| *f > 0)
        .collect();

    overlaps.len()
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    let ans1 = part1(input.to_owned());
    let ans2 = part2(input);

    println!("ans1 {ans1}"); // 413
    println!("ans2 {ans2}"); // 806
}

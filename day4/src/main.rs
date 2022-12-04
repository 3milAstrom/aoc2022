extern crate array_tool;
use array_tool::vec::Intersect;

use std::fs;

struct Sequence {
    start: i32,
    stop: i32,
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn part1(input: Vec<String>) -> usize {
    let overlaps: Vec<i32> = input
        .iter()
        .map(|f| {
            let seq: Vec<Sequence> = f
                .split(',')
                .map(|s| {
                    let st = s.to_string();
                    let val: Vec<i32> = st.split('-').map(|v| v.parse::<i32>().unwrap()).collect();

                    Sequence {
                        start: val.first().unwrap().to_owned(),
                        stop: val.get(1).unwrap().to_owned(),
                    }
                })
                .collect();

            let s1 = seq.first().unwrap().to_owned();
            let s2 = seq.get(1).unwrap().to_owned();

            let first_seq: Vec<i32> = (s1.start..(s1.stop + 1)).collect();
            let second_seq: Vec<i32> = (s2.start..(s2.stop + 1)).collect();

            let intersect = first_seq.intersect(second_seq.to_owned());

            if intersect.len() == first_seq.len() || intersect.len() == second_seq.len() {
                1
            } else {
                0
            }
        })
        .filter(|f| *f != 0)
        .collect();

    overlaps.len()
}

fn part2(input: Vec<String>) -> usize {
    let overlaps: Vec<usize> = input
        .iter()
        .map(|f| {
            let seq: Vec<Sequence> = f
                .split(',')
                .map(|s| {
                    let st = s.to_string();
                    let val: Vec<i32> = st.split('-').map(|v| v.parse::<i32>().unwrap()).collect();

                    Sequence {
                        start: val.first().unwrap().to_owned(),
                        stop: val.get(1).unwrap().to_owned(),
                    }
                })
                .collect();

            let s1 = seq.first().unwrap().to_owned();
            let s2 = seq.get(1).unwrap().to_owned();

            let first_seq: Vec<i32> = (s1.start..(s1.stop + 1)).collect();
            let second_seq: Vec<i32> = (s2.start..(s2.stop + 1)).collect();

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

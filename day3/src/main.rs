extern crate array_tool;
use array_tool::vec::Intersect;

use std::fs;

fn read_file(path: &str) -> Vec<String>{
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect()
}

fn part1(input: Vec<String>) -> Vec<i32> {
    let ans: Vec<i32> = input.iter().map(|s| {
        let lenght = s.len();
        let (f,l) = s.split_at(lenght / 2);

        let l1: Vec<char> = f.chars().collect();
        let l2: Vec<char> = l.chars().collect();
        let i = l1.intersect(l2);
        let val = i.first().unwrap().to_owned();


        let a = match val {
            n @ 'a'..='z' => n as i32 - 96,
            n @ 'A'..='Z' => n as i32 - 64 + 26,
            n => panic!("{n}")
        };
        a
    }).collect();

    ans
}

fn part2(input: Vec<String>) -> Vec<i32> {

    let len = input.len();
    let mut i = 0;
    let mut v: Vec<i32> = Vec::new();

    while i < len {
        let s1: Vec<char> = input.get(i).unwrap().chars().collect();
        let s2: Vec<char> = input.get(i + 1).unwrap().chars().collect();
        let s3: Vec<char> = input.get(i + 2).unwrap().chars().collect();

        let val = s1.intersect(s2).intersect(s3).first().unwrap().to_owned();

        match val {
            n @ 'a'..='z' => v.push(n as i32 - 96),
            n @ 'A'..='Z' => v.push(n as i32 - 64 + 26),
            n => panic!("{n}")
        };
        i += 3;
    }

    v
}


fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    let ans = part1(input.to_owned()).iter().sum::<i32>();
    let ans2 = part2(input).iter().sum::<i32>();

    println!("part1: {ans}"); // 7727
    println!("part2: {ans2}"); // 2609

}


use std::{fs, collections::{HashSet, HashMap}};
fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    let rows: Vec<String>= content.split('\n').map(|s| s.to_string()).collect();
    rows
}

fn part1(input: Vec<String>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut cycles: i32 = 1;
    let mut registry = 1;
    map.insert(0, registry);

    input.iter().for_each(|f| {
        let asd = f.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        let o = asd[0].to_owned();
        match o.as_str() {
            "addx" => {
                let v = asd[1].to_owned().parse::<i32>().unwrap();
                cycles += 1;
                map.insert(cycles, registry);
                cycles += 1;
                registry += v;
                map.insert(cycles, registry);

            }
            "noop" => {
                cycles += 1;
                map.insert(cycles, registry);
            }
            _ => panic!()
        };
    });

    let t20 = map.get(&20).unwrap() * 20;
    let t60 = map.get(&60).unwrap() * 60;
    let t100 = map.get(&100).unwrap() * 100;
    let t140 = map.get(&140).unwrap() * 140;
    let t180 = map.get(&180).unwrap() * 180;
    let t220 = map.get(&220).unwrap() * 220;


    let s = t20 + t60 + t100 + t140 + t180 + t220;


    println!();
}


fn part2(input: Vec<String>) {
    let mut crt_col = 0;
    let mut crt_row = 0;

    let mut crt: Vec<Vec<String>> = (0..40).map(|_| {
        (0..40).map(|_| {
            '.'.to_string()

        }).collect::<Vec<String>>()
    }).collect();

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut cycles: i32 = 1;
    let mut registry = 1;
    map.insert(0, registry);

    input.iter().for_each(|f| {
        let asd = f.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        let o = asd[0].to_owned();
        match o.as_str() {
            "addx" => {
                let v = asd[1].to_owned().parse::<i32>().unwrap();
                cycles += 1;
                map.insert(cycles, registry);
                cycles += 1;
                registry += v;
                map.insert(cycles, registry);

            }
            "noop" => {
                cycles += 1;
                map.insert(cycles, registry);
            }
            _ => panic!()
        };
    });

    let t20 = map.get(&20).unwrap() * 20;
    let t60 = map.get(&60).unwrap() * 60;
    let t100 = map.get(&100).unwrap() * 100;
    let t140 = map.get(&140).unwrap() * 140;
    let t180 = map.get(&180).unwrap() * 180;
    let t220 = map.get(&220).unwrap() * 220;


    let s = t20 + t60 + t100 + t140 + t180 + t220;


    println!();
}

fn main() {
    let input = read_file("./src/test_input.txt");
    // let input = read_file("./src/part1_input.txt");
    part2(input);
    println!("Hello, world!");
}

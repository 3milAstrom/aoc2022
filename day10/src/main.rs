
use std::{fs, collections:: HashMap};
fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    let rows: Vec<String>= content.split('\n').map(|s| s.to_string()).collect();
    rows
}

fn part1(input: Vec<String>) -> HashMap<i32, i32>{
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut cycles: i32 = 1;
    let mut registry = 1;
    map.insert(1, registry);

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


    println!("{s}"); // 14240
    map
}


fn part2(map: HashMap<i32, i32>) {
    let mut index = 1;
    let crt: Vec<Vec<String>> = (0..6).map(|_| {
        (0..40).map(|col| {
            let v = map.get(&index).unwrap();
            let sign = if ((v % 40) - col).abs() <=1 {
                '#'.to_string()
            }else {
                '.'.to_string()
            };
            index += 1;
            sign
        }).collect::<Vec<String>>()
    }).collect();

    crt.iter().for_each(|row| {
        row.iter().for_each(|v| {
            print!("{v}");
        });
        println!();
    });

    println!(); // PLULKBZH
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    let part1 = part1(input);
    part2(part1);
}

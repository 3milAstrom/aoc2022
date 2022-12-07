use std::fs;
use std::collections::HashMap;

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn run() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");

    let mut file_system: HashMap<String, usize> = HashMap::new();
    let mut current_path: String = "".to_string();
    let mut current_l: Vec<usize> = Vec::new();
    input.iter().for_each(|r| {
        let (operation, value) = (&r[0..=3],&r[4..]);
        match operation {
            "$ cd" => {
                if value.trim() == ".." {
                    let l = current_l.pop().unwrap();
                    let next = current_path[..current_path.len() - l ].to_string().clone();
                    *file_system.get_mut(&next).unwrap() += *file_system.get_mut(&current_path).unwrap();
                    current_path = next;
                } else {
                    current_l.push(value.trim().len());
                    current_path.push_str(value.trim());
                    file_system.insert(current_path.to_owned(), 0);
                }

            },
            "$ ls" => (),
            _ => {
                let (s,_) = r.split_once(' ').unwrap();
                if s != "dir".to_string() {
                    let size = s.parse::<usize>().unwrap();
                    *file_system.get_mut(&current_path).unwrap() += size;
                }
            }

        }
    });

    while current_path != "/" {
        let l = current_l.pop().unwrap();
        let next = current_path[..current_path.len() - l ].to_string().clone();
        *file_system.get_mut(&next).unwrap() += *file_system.get_mut(&current_path).unwrap();
        current_path = next;
    }
    let mut size: usize = 0;
    let current: i32 = 70000000 - (*file_system.get("/").unwrap() as i32);
    let mut v: Vec<i32> = Vec::new();

    for (key, value) in file_system.to_owned() {
        if value < 100000 {
            size += value;
        }
        let val = value as i32;
        if current + val >= 30000000 {
            v.push(val);
        }
    }

    v.sort();

    println!("part1: {size}"); // Part1 1182909
    println!("part2: {:}", v.first().unwrap()); // Part2 2832508

}

fn main() {
    run();
}

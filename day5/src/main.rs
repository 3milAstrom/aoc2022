use std::{collections::VecDeque, fs};

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

struct Crate {
    sign: String,
}

fn setup_cargo(input: Vec<String>) -> (Vec<VecDeque<Crate>>, Vec<String>) {
    let mut rows = input
        .iter()
        .map_while(|s| {
            if s.is_empty() {
                return None;
            }
            let row = s
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.get(1).unwrap().to_string())
                .collect::<Vec<String>>();
            Some(row)
        })
        .collect::<Vec<Vec<String>>>();

    let move_start = rows.len();
    rows.remove(rows.len() - 1);

    let cargo_width = rows.first().unwrap().len();

    let mut cargo: Vec<VecDeque<Crate>> = Vec::new();
    let mut i = 0;
    while i < cargo_width {
        cargo.push(VecDeque::new());
        i += 1;
    }

    rows.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(column, c)| {
            if !c.trim().is_empty() {
                cargo[column].push_front(Crate { sign: c.to_owned() })
            }
        });
    });

    let moves = input
        .to_owned()
        .drain(move_start + 1..input.len())
        .collect::<Vec<String>>();

    (cargo, moves)
}

fn part1(input: Vec<String>) {
    let (mut cargo, moves) = setup_cargo(input);;
    moves.iter().for_each(|s| {
        let c: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
        let amount = c.get(1).unwrap().parse::<i32>().unwrap();
        let from = c.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = c.get(5).unwrap().parse::<usize>().unwrap() - 1;

        let mut j = 0;

        while j < amount {
            let c = cargo.get_mut(from).unwrap().pop_back().unwrap();
            cargo.get_mut(to).unwrap().push_back(c);

            j += 1;
        }
    });

    for ele in cargo {
        print!("{:}", ele.back().unwrap().sign);
    }
}

fn part2(input: Vec<String>) {
    let (mut cargo, moves) = setup_cargo(input);

    moves.iter().for_each(|s| {
        let c: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
        let amount = c.get(1).unwrap().parse::<i32>().unwrap();
        let from = c.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = c.get(5).unwrap().parse::<usize>().unwrap() - 1;

        let mut j = 0;
        let mut stage: VecDeque<Crate> = VecDeque::new();

        while j < amount {
            let c = cargo.get_mut(from).unwrap().pop_back().unwrap();
            stage.push_back(c);

            j += 1;
        }

        j = 0;
        while j < amount {
            let c = stage.pop_back().unwrap();
            cargo.get_mut(to).unwrap().push_back(c);
            j += 1;
        }
    });

    for ele in cargo {
        print!("{:}", ele.back().unwrap().sign);
    }
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part1(input.to_owned());
    println!();
    part2(input);
    println!();
}

use std::num::ParseIntError;
use std::str::FromStr;
use std::{fs};

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

struct Move {
    amount: i32,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<&str> = s.split(' ').collect::<Vec<&str>>();
        let amount = c.get(1).unwrap().parse::<i32>()?;
        let from = c.get(3).unwrap().parse::<usize>()? - 1;
        let to = c.get(5).unwrap().parse::<usize>()? - 1;

        Ok(Move { amount, from, to })
    }
}

fn setup_cargo(input: Vec<String>) -> (Vec<Vec<String>>, Vec<Move>) {
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
                .map(|c| {
                    let s = c.get(1).unwrap().to_string();
                    if !s.trim().is_empty() {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<String>>>();
            Some(row)
        })
        .collect::<Vec<Vec<Option<String>>>>();

    let move_start = rows.len();
    rows.remove(rows.len() - 1);

    let cargo_width = rows.first().unwrap().len();

    let mut cargo: Vec<Vec<String>> = vec![Vec::new(); cargo_width];

    rows.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(column, c)| {
            if c.is_some() {
                cargo[column].push(c.to_owned().unwrap())
            }
        });
    });

    cargo.iter_mut().for_each(|v| {
        v.reverse();
    });

    let moves = input
        .to_owned()
        .drain(move_start + 1..input.len())
        .map(|s| s.parse::<Move>().unwrap())
        .collect::<Vec<Move>>();

    (cargo, moves)
}

fn part1(input: Vec<String>) {
    let (mut cargo, moves) = setup_cargo(input);
    moves.iter().for_each(|m| {
        let mut j = 0;

        while j < m.amount {
            let c = cargo.get_mut(m.from).unwrap().pop().unwrap();
            cargo.get_mut(m.to).unwrap().push(c);

            j += 1;
        }
    });

    for ele in cargo {
        print!("{:}", ele.last().unwrap());
    }
}

fn part2(input: Vec<String>) {
    let (mut cargo, moves) = setup_cargo(input);

    moves.iter().for_each(|m| {
        let mut j = 0;
        let mut stage: Vec<String> = Vec::new();

        while j < m.amount {
            let c = cargo.get_mut(m.from).unwrap().pop().unwrap();
            stage.push(c);

            j += 1;
        }

        j = 0;

        while j < m.amount {
            let c = stage.pop().unwrap();
            cargo.get_mut(m.to).unwrap().push(c);
            j += 1;
        }
    });

    for ele in cargo {
        print!("{:}", ele.last().unwrap());
    }
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part1(input.to_owned()); // NTWZZWHFV
    println!();
    part2(input); // BRZGFVBTJ
    println!();
}

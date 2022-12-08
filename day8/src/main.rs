use std::{collections::HashMap, fs};

struct Directions {
    top: Vec<String>,
    bottom: Vec<String>,
    left: Vec<String>,
    right: Vec<String>,
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn get_view(val: i32, v: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while j < v.len() {
        i += 1;
        if val <= v[j] {
            break;
        }
        j += 1;
    }
    i
}

fn get_column(v: Vec<Vec<i32>>, col: i32) -> Vec<i32> {
    v.iter().map(| row| {
        row[col as usize]
    }).collect::<Vec<i32>>()

}

fn part1(input: Vec<String>) {
    let asd = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sol: Vec<i32> = Vec::new();
    asd.iter().enumerate().for_each(|(i,row)| {
        row.iter().enumerate().for_each(|(j, val)|{
            let col = get_column(asd.to_owned(), j as i32);
            if (i != 0 && j != 0) && (i != col.len() - 1 && j != row.len()- 1) {
                let top: i32 = col[0..i].iter().rev().map(|f| f.to_owned()).max().unwrap();
                let bottom = col[i + 1..col.len()].iter().map(|f| f.to_owned()).max().unwrap();
                let left = row[0..j].iter().rev().map(|f| f.to_owned()).max().unwrap();
                let right = row[j + 1..row.len()].iter().map(|f| f.to_owned()).max().unwrap();

                if val > &top || val > &bottom || val > &left || val > &right {
                    sol.push(val.to_owned());
                }
            }

        })
    });

    let row_len = asd.first().unwrap().len();
    let coll_len = asd.len();

    let l = sol.iter().len() + (row_len * 2) + (coll_len * 2) - 4;
    println!("{l}") // 1715
}


fn part2(input: Vec<String>) {
    let asd = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sol: Vec<i32> = Vec::new();
    asd.iter().enumerate().for_each(|(i,row)| {
        row.iter().enumerate().for_each(|(j, val)|{
            let col = get_column(asd.to_owned(), j as i32);
            if (i != 0 && j != 0) && (i != col.len() - 1 && j != row.len()- 1) {
                let top = col[0..i].iter().rev().map(|f| f.to_owned()).collect::<Vec<i32>>();
                let bottom = col[i + 1..col.len()].iter().map(|f| f.to_owned()).collect::<Vec<i32>>();
                let left = row[0..j].iter().rev().map(|f| f.to_owned()).collect::<Vec<i32>>();
                let right = row[j + 1..row.len()].iter().map(|f| f.to_owned()).collect::<Vec<i32>>();

                let a = get_view(val.to_owned(), top.to_owned());
                let b = get_view(val.to_owned(), bottom).to_owned();
                let c = get_view(val.to_owned(), left.to_owned());
                let d = get_view(val.to_owned(), right.to_owned());
                sol.push(a * b * c * d);
            }

        })
    });

    let row_len = asd.first().unwrap().len();
    let coll_len = asd.len();

    let s = sol.iter().max().unwrap();
    println!("{s}") // 374400
}


fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part2(input);
}

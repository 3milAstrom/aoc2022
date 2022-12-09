use std::{fs, collections::HashSet};
fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn adjacent(&self, p2: Point) -> bool {

        let x = i32::abs(p2.x - self.x);
        let y = i32::abs(p2.y - self.y);

        x <= 1 && y <=1
    }

}


fn part1(input: Vec<String>) {
    let mut head = Point{
        x: 0,
        y: 0
    };

    let mut tail = Point {
        x: 0,
        y: 0
    };

    let mut unique_points: HashSet<(i32,i32)> = HashSet::new();
    unique_points.insert((tail.x, tail.y));

    let mut last_point = head.clone();

    input.iter().for_each(|r| {
        let (dirr, n_s) = r.split_once(' ').unwrap();
        let n = n_s.parse::<i32>().unwrap();

        match dirr {
            "R" => {
                (0..n).for_each(|_| {
                    last_point = head.clone();
                    head.x += 1;
                    if !tail.adjacent(head) {
                        tail = last_point;
                        unique_points.insert((tail.x, tail.y));
                    }
                })
            },
            "L" => {
                (0..n).for_each(|_| {
                    last_point = head.clone();
                    head.x -= 1;
                    if !tail.adjacent(head) {
                        tail = last_point;
                        unique_points.insert((tail.x, tail.y));
                    }
                })

            },
            "U" => {
                (0..n).for_each(|_| {
                    last_point = head.clone();
                    head.y += 1;
                    if !tail.adjacent(head) {
                        tail = last_point;
                        unique_points.insert((tail.x, tail.y));
                    }
                })
            },
            "D" => {
                (0..n).for_each(|_| {
                    last_point = head.clone();
                    head.y -= 1;
                    if !tail.adjacent(head) {
                        tail = last_point;
                        unique_points.insert((tail.x, tail.y));
                    }
                })
            },
            n => panic!("Panic {n}")
        };
    });

    let s =unique_points.len();

    println!("{s}");

}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part1(input);
}

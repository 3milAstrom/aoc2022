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

impl ToString for Point {
    fn to_string(&self) -> String {
        let mut x = self.x.to_string();
        x.push_str(",");
        let y = self.y.to_string();
        x.push_str(y.as_str());
        x
    }
}

impl Point {
    fn adjacent(&self, p2: Point) -> bool {

        let x = i32::abs(p2.x - self.x);
        let y = i32::abs(p2.y - self.y);

        x <= 1 && y <=1
    }

    fn move_self(&mut self, p2: Point) {
        let x = i32::abs(p2.x - self.x);
        let y = i32::abs(p2.y - self.y);
        if x > 1 || y > 1 {
            if p2.x > self.x {
                self.x+=1;
            } else if x > 0{
                self.x-=1;
            }
            if p2.y > self.y {
                self.y+=1;
            } else if y > 0 {
                self.y-=1;
            }
        }
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

    println!("{s}"); //5878

}

fn part2(input: Vec<String>) {
    let mut head = Point{
        x: 0,
        y: 0
    };
    let mut knots: Vec<Point> = (0..9).map(|_|{
        Point{
            x: 0,
            y: 0
        }
    }).collect();

    let mut unique_points: HashSet<String> = HashSet::new();
    // unique_points.insert("0,0".to_string());

    input.iter().for_each(|r| {
        let (dirr, n_s) = r.split_once(' ').unwrap();
        let n = n_s.parse::<i32>().unwrap();

        match dirr {
            "R" => {
                (0..n).for_each(|_| {
                    head.x += 1;
                    let mut tmp_head =head.clone();
                    if !knots[0].adjacent(head) {
                        (0..knots.len()).for_each(|i| {
                            knots[i].move_self(tmp_head);
                            tmp_head = knots[i];
                        });
                    }
                    unique_points.insert(knots[8].to_string());
                })
            },
            "L" => {
                (0..n).for_each(|_| {
                    head.x -= 1;
                    let mut tmp_head = head.clone();
                    if !knots[0].adjacent(head) {
                        (0..knots.len()).for_each(|i| {
                            knots[i].move_self(tmp_head);
                            tmp_head = knots[i];
                        });
                    }
                    unique_points.insert(knots[8].to_string());
                })

            },
            "U" => {
                (0..n).for_each(|_| {
                    head.y += 1;
                    let mut tmp_head = head.clone();
                    if !knots[0].adjacent(head) {
                        (0..knots.len()).for_each(|i| {
                            knots[i].move_self(tmp_head);
                            tmp_head = knots[i];

                        });
                    }
                    unique_points.insert(knots[8].to_string());
                })
            },
            "D" => {
                (0..n).for_each(|_| {
                    head.y -= 1;
                    let mut tmp_head = head.clone();
                    if !knots[0].adjacent(head) {
                        (0..knots.len()).for_each(|i| {
                            knots[i].move_self(tmp_head);
                            tmp_head = knots[i];
                        });
                    }
                    unique_points.insert(knots[8].to_string());
                })
            },
            n => panic!("Panic {n}")
        };
    });

    let s =unique_points.len();

    println!("{s}");

}

fn main() {
    // let input = read_file("./src/test2_input.txt");
    let input = read_file("./src/part1_input.txt");
    part1(input.to_owned());
    part2(input);
}

use std::{fs, collections::HashMap};

struct Directions {
    top: Vec<String>,
    bottom: Vec<String>,
    left: Vec<String>,
    right: Vec<String>
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn to_key(i: i32, j: i32) -> String {
    let mut is = i.to_string();
    let js = j.to_string();
    is.push_str(" ");
    is.push_str(js.as_str());
    return is
}

fn get_indexes_to_check(pos: (i32,i32), map: HashMap<String, i32>, row_len: usize, coll_len: usize) -> Directions {
    let top: Vec<String> = (0..pos.0).map(|v| {
        to_key(v, pos.1)
    }).collect();

    let bottom: Vec<String> = (pos.0 + 1..coll_len as i32).map(|v| {
        to_key(v, pos.1)
    }).collect();

    let left: Vec<String> = (0..pos.1).map(|v| {
        to_key(pos.0, v)
    }).collect();

    let right: Vec<String> = (pos.1+1..row_len as i32).map(|v| {
        to_key(pos.0, v)
    }).collect();

    println!("");
    Directions {
        top,
        bottom,
        left,
        right
    }
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
    println!();
    i
}

fn part1(input: Vec<String>) {
    let asd = input.iter().map(|s| s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut map: HashMap<String,i32>= HashMap::new();
    asd.iter().enumerate().for_each(|(i,v)| {
        v.iter().enumerate().for_each(|(j,val)| {
            let k = to_key(i as i32, j as i32);

            map.insert(k, val.to_owned());
        });

    });

    let row_len = asd.first().unwrap().len();
    let coll_len = asd.len();

    let mut sol: Vec<i32> = Vec::new();


    asd.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j,val)| {
            if (i != 0 && j != 0) && (i != coll_len - 1 && j != row_len - 1) {
                let p = to_key(i as i32, j as i32);
                // let val = map.get(&p).unwrap();

                let dirr = get_indexes_to_check((i as i32,j as i32), map.to_owned(), row_len, coll_len);
                let top_max = dirr.top.iter().map(|f| {
                    map.get(f).unwrap()
                }).max().unwrap();
                let bottom_max = dirr.bottom.iter().map(|f| {
                    map.get(f).unwrap()
                }).max().unwrap();
                let left_max = dirr.left.iter().map(|f| {
                    map.get(f).unwrap()
                }).max().unwrap();
                let right_max = dirr.right.iter().map(|f| {
                    map.get(f).unwrap()
                }).max().unwrap();

                if val > top_max || val > bottom_max || val > left_max || val > right_max {
                    sol.push(val.to_owned());
                }
            }

        });
    });

    let s = sol.iter().len() + (row_len * 2) + (coll_len * 2) - 4;
    println!("{s}") // 1715
}

fn part2(input: Vec<String>) {
    let asd = input.iter().map(|s| s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut map: HashMap<String,i32>= HashMap::new();
    asd.iter().enumerate().for_each(|(i,v)| {
        v.iter().enumerate().for_each(|(j,val)| {
            let k = to_key(i as i32, j as i32);

            map.insert(k, val.to_owned());
        });

    });

    let row_len = asd.first().unwrap().len();
    let coll_len = asd.len();

    let mut sol: Vec<i32> = Vec::new();


    asd.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j,val)| {
            if (i != 0 && j != 0) && (i != coll_len - 1 && j != row_len - 1) {
                let p = to_key(i as i32, j as i32);
                // let val = map.get(&p).unwrap();

                let dirr = get_indexes_to_check((i as i32,j as i32), map.to_owned(), row_len, coll_len);
                let mut top_max = dirr.top.iter().map(|f| {
                    map.get(f).unwrap().to_owned()
                }).collect::<Vec<i32>>();
                let bottom_max = dirr.bottom.iter().map(|f| {
                    map.get(f).unwrap().to_owned()
                }).collect::<Vec<i32>>();
                let mut left_max = dirr.left.iter().map(|f| {
                    map.get(f).unwrap().to_owned()
                }).collect::<Vec<i32>>();
                let right_max = dirr.right.iter().map(|f| {
                    map.get(f).unwrap().to_owned()
                }).collect::<Vec<i32>>();

                top_max.reverse();
                left_max.reverse();

                let a = get_view(val.to_owned(), top_max.to_owned());
                let b = get_view(val.to_owned(), bottom_max.to_owned());
                let c = get_view(val.to_owned(), left_max.to_owned());
                let d = get_view(val.to_owned(), right_max.to_owned());
                let aa = a * b * c * d;
                sol.push(a * b * c * d);
            }

        });
    });

    let s = sol.iter().max().unwrap();
    println!("{s}") // 374400
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part2(input);
}
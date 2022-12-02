use std::fs;

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| {
        let a: Vec<i32> = s.split(' ').map(|f|{
            match f {
                "A"|"X" => 1,
                "B"|"Y" => 2,
                "C"|"Z" => 3,
                _ => panic!("Wrong input {f}")
            }
        }).collect();
        return a;
    }).collect()
}


fn part1(input: Vec<Vec<i32>>) -> i32 {
    let asd: Vec<i32> = input.iter().map(|f| {
        let (o, p) = (f.get(0).unwrap().to_owned(), f.get(1).unwrap().to_owned());
        if o == p {
            return 3 + p
        }

        if o == 1 {
            if p == 2 {
                return 6 + p
            }

            if p == 3 {
                return 0 + p
            }
        }

        if o == 2 {
            if p == 1 {
                return 0 + p
            }
            if p == 3 {
                return 6 + p
            }
        }

        if o == 3 {
            if p == 1 {
                return 6 + p
            }

            if p == 2 {
                return 0 + p
            }
        }

        panic!("")

    }).collect();

    // println!("{:?}", asd);

    asd.iter().sum()
}


fn part2(input: Vec<Vec<i32>>) -> i32 {
    let asd: Vec<i32> = input.iter().map(|f| {
        let (o, p) = (f.get(0).unwrap().to_owned(), f.get(1).unwrap().to_owned());

        let re = match p {
            1 => (o + 2)%3 + 0,
            2 => o + 3,
            3 => (o + 1)%3 + 6,
            _ => panic!("")
        };
        if o == p {
            return 3 + p
        }

        if o == 1 {
            if p == 2 {
                return 6 + p
            }

            if p == 3 {
                return 0 + p
            }
        }

        if o == 2 {
            if p == 1 {
                return 0 + p
            }
            if p == 3 {
                return 6 + p
            }
        }

        if o == 3 {
            if p == 1 {
                return 6 + p
            }

            if p == 2 {
                return 0 + p
            }
        }

        panic!("")

    }).collect();

    // println!("{:?}", asd);

    // asd.iter().sum()

    0
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");

    let sum = part1(input);
    println!("sum: {sum}"); //11873

}

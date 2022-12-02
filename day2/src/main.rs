use std::fs;

fn read_file(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| {
        let a: Vec<i32> = s.split(' ').map(|f|{
            match f {
                "A"|"X" => 0,
                "B"|"Y" => 1,
                "C"|"Z" => 2,
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
            return 3 + (p + 1)
        }

        if o == 0 {
            if p == 1 {
                return 6 + (p + 1)
            }

            if p == 2 {
                return 0 + (p + 1)
            }
        }

        if o == 1 {
            if p == 0 {
                return 0 + (p + 1)
            }
            if p == 2 {
                return 6 + (p + 1)
            }
        }

        if o == 2 {
            if p == 0 {
                return 6 + (p + 1)
            }

            if p == 1 {
                return 0 + (p + 1)
            }
        }

        panic!("")

    }).collect();

    asd.iter().sum()
}


fn part2(input: Vec<Vec<i32>>) -> i32 {
    let asd: Vec<i32> = input.iter().map(|f| {
        let (o, p) = (f.get(0).unwrap().to_owned(), f.get(1).unwrap().to_owned());

        let re = match p {
            0 => ((o + 2)%3 + 1) + 0,
            1 => (o + 1) + 3,
            2 => ((o + 1)%3 + 1) + 6,
            _ => panic!("")
        };

        return re

    }).collect();

    asd.iter().sum()
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");

    let sum = part1(input.to_owned());
    let sum2 = part2(input.to_owned());
    println!("sum: {sum}"); //11873
    println!("sum2: {sum2}"); //12014

}

use std::fs;
use std::str::FromStr;

// struct Chunk {
//     max: i32,
//     current: i32
// }

fn read_file(path: &str) -> Vec<String>{
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect()
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/input_part1.txt");
    let mut v = Vec::new();
    v.push(0);

    let mut d = input.iter().fold(v, |mut acc, i| {
        if i.len() == 0  {
            acc.push(0);
            return acc;
        }

        let value: i32 = FromStr::from_str(i).unwrap();

        let a = acc.last_mut().unwrap();
        *a = *a + value;
        acc
    });
    d.sort();
    d.reverse();
    let biggest = d.first().unwrap();
    let sum: i32 = d[0..3].iter().sum();

    println!("Biggest value: {biggest}");
    println!("Top three: {}", sum);
}

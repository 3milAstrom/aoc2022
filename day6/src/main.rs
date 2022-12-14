use std::fs;

fn read_file(path: &str) -> String {
    return fs::read_to_string(path).expect("Error")
}

fn get_slice(s: Vec<char>, start: usize, stop: usize) -> Vec<char> {
    s[start..stop].iter().map(|s| s.to_owned()).collect::<Vec<char>>()
}

fn find_pos(input: String, n_unique: usize) -> Option<usize> {
    let chars = input.chars().collect::<Vec<char>>();
    chars.iter().enumerate().position(|(i, _)| {
        if i > chars.len() - n_unique {
            return false
        }
        let slice = get_slice(chars.to_owned(), i, i+n_unique);
        slice.iter().all(|&f| {
            let d = slice.iter().filter(|&&x| x == f).collect::<Vec<&char>>().len();
            if d > 1 {false} else {true}
        })
    })
}

fn part1(input: String) {
    let pos = find_pos(input, 4);
    if pos.is_some() {println!("{:}", pos.unwrap() + 4)} else {println!("")}
}

fn part2(input: String) {
    let pos = find_pos(input, 14);
    if pos.is_some() {println!("{:}", pos.unwrap() + 14)} else {println!("")}
}

fn main() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");
    part1(input.to_owned()); // 1210
    part2(input); // 3476

}

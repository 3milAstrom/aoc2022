use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;

struct FileSystem {
    map: HashMap<String, usize>,
    current_path: RefCell<Vec<String>>
}

impl FileSystem {
    fn get_current_folder(&self) -> String {
        self.current_path.borrow().concat()
    }

    fn get_size(&mut self, dirr: String) -> usize {
        self.map.get(&dirr).unwrap().to_owned()
    }

    fn go_up(&mut self) {
        let c1 = self.get_current_folder();
        self.current_path.borrow_mut().pop();
        let c2 = self.get_current_folder();
        let s1 = self.get_size(c1);
        let s2 = self.get_size(c2.to_owned());
        self.map.insert(c2, s1 + s2);
    }

    fn go_in(&mut self, path: String) {
        self.current_path.borrow_mut().push(path.trim().to_string());
        self.map.insert(self.get_current_folder(), 0);
    }

    fn add_size(&mut self, size: usize) {
        let s1 = self.get_size(self.get_current_folder());
        self.map.insert(self.get_current_folder(), s1 + size);
    }

    fn go_to_root(&mut self) {
        while self.get_current_folder() != "/" {
            self.go_up();
        }
    }
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Error");
    return content.split('\n').map(|s| s.to_string()).collect();
}

fn run() {
    // let input = read_file("./src/test_input.txt");
    let input = read_file("./src/part1_input.txt");

    let mut file_system = FileSystem {
        map: HashMap::new(),
        current_path: RefCell::new(Vec::new())
    };

    input.iter().for_each(|r| {
        let (operation, value) = (&r[0..=3],&r[4..]);
        match operation {
            "$ cd" => {
                if value.trim() == ".." {
                    file_system.go_up();
                } else {
                    file_system.go_in(value.to_string());
                }

            },
            "$ ls" => (),
            _ => {
                let (s,_) = r.split_once(' ').unwrap();
                if s != "dir".to_string() {
                    let size = s.parse::<usize>().unwrap();
                    file_system.add_size(size)
                }
            }

        }
    });

    file_system.go_to_root();

    let mut size: usize = 0;
    let current: i32 = 70000000 - (*file_system.map.get("/").unwrap() as i32);
    let mut v: Vec<i32> = Vec::new();

    for (_, value) in file_system.map.to_owned() {
        if value < 100000 {
            size += value;
        }
        let val = value as i32;
        if current + val >= 30000000 {
            v.push(val);
        }
    }

    v.sort();

    println!("part1: {size}"); // Part1 1182909
    println!("part2: {:}", v.first().unwrap()); // Part2 2832508

}

fn main() {
    run();
}

use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let strings: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!("{}", strings.len());
}

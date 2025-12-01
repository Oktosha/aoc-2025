use regex::Regex;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut dial = 50;
    let mut answer = 0;
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let re = Regex::new(r"([LR])([0-9]+)").unwrap();
        let captures = re.captures(&line).unwrap();
        let direction = captures.get(1).unwrap().as_str();
        let clicks = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        if direction == "R" {
            dial = (dial + clicks) % 100;
        } else {
            dial = (dial + 100 - clicks) % 100;
        }
        if dial == 0 {
            answer += 1;
        }
    }
    println!("{}", answer);
}

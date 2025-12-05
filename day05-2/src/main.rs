use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let mut events: Vec<(usize, usize)> = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.contains("-") {
            let vals = line.split("-").collect::<Vec<&str>>();
            events.push((vals[0].parse::<usize>().unwrap(), 0));
            events.push((vals[1].parse::<usize>().unwrap() + 1, 1));
        }
    }

    events.sort();
    println!("{:?}", events);
    let mut ans = 0;
    let mut balance = 0;
    let mut prev = events[0].0;
    for e in &events {
        if balance > 0 {
            ans += e.0 - prev;
        }
        if e.1 == 0 {
            balance += 1;
        } else {
            balance -= 1;
        }
        prev = e.0;
    }
    println!("{}", ans);
}

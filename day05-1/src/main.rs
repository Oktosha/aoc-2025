use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.contains("-") {
            let vals = line.split("-").collect::<Vec<&str>>();
            ranges.push((vals[0].parse().unwrap(), vals[1].parse().unwrap()));
        } else if !line.is_empty() {
            ids.push(line.parse().unwrap());
        }
    }

    let mut ans = 0;
    'outer: for id in &ids {
        for (l, r) in &ranges {
            if l <= id && id <= r {
                ans += 1;
                continue 'outer;
            }
        }
    }

    println!("{}", ans);
}

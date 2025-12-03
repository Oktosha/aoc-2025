use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let mut ans = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap().chars().collect::<Vec<char>>();
        let mut first_digit_position = line.len() - 2;
        for i in (0..(line.len() - 2)).rev() {
            if (line[i] >= line[first_digit_position]) {
                first_digit_position = i;
            }
        }
        let mut second_digit_position = first_digit_position + 1;
        for i in first_digit_position + 1..line.len() {
            if (line[i] >= line[second_digit_position]) {
                second_digit_position = i;
            }
        }
        let jolts = line[first_digit_position].to_digit(10).unwrap() * 10
            + line[second_digit_position].to_digit(10).unwrap();
        ans += jolts;
        println!("{}", jolts);
    }
    println!("{}",ans);
}

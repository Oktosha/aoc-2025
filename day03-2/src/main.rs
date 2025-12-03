use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let mut ans = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap().chars().collect::<Vec<char>>();
        let mut digits = Vec::new();
        let mut previous_digit_position:i32 = -1;
        for i in 1_i32..=12_i32 {
            let number_of_remaining_digits = 12 - i;
            let mut digit_position = previous_digit_position + 1;
            for j in ((previous_digit_position + 1)..(line.len() as i32 - number_of_remaining_digits)) {
                if (line[j as usize] > line[digit_position as usize]) {
                    digit_position = j;
                }
            }
            digits.push(line[digit_position as usize]);
            previous_digit_position = digit_position;
        }
        let jolts = digits.iter().collect::<String>().parse::<i64>().unwrap();
        ans += jolts;
        println!("{}", jolts);
    }
    println!("{}",ans);
}

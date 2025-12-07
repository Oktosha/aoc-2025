use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let lines = std::io::BufReader::new(file).lines().collect::<Vec<_>>();
    let num_len = lines.len() - 1;
    let (num_strs, action_str) = lines.split_at(num_len);
    let nums = num_strs.into_iter().map({|item| {
        let item = item.as_ref().unwrap();
        item.split(" ").
            filter(|e| !e.is_empty())
            .map(|e| {
                e.parse::<i64>().unwrap()}).collect::<Vec<i64>>()
    }}).collect::<Vec<Vec<i64>>>();
    let last_line = action_str[0].as_ref().unwrap();
    let actions = last_line.split(" ").filter(|e| {!e.is_empty()}).collect::<Vec<_>>();
    let mut ans = 0;
    for (pos, action) in actions.into_iter().enumerate() {
        if action == "+" {
            for i in 0..nums.len() {
                ans += nums[i][pos];
            }
        }
        else {
            let mut result = 1;
            for i in 0..nums.len() {
                result *= nums[i][pos];
            }
            ans += result;
        }
    }
    println!("{:?}", ans);
}

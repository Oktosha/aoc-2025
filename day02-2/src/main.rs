use std::fs;
use std::collections::HashSet;

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let ranges = data.split(",");
    let mut ans = 0;
    for range in ranges {
        let mut invalid_ids = HashSet::new();
        let mut a = range.split("-");
        let start = a.next().unwrap().parse::<i64>().unwrap();
        let end = a.next().unwrap().parse::<i64>().unwrap();
        println!("{:?}-{:?}", start, end);
        for number_length in 2..12 {
            for group_length in 1..6 {
                let number_of_groups = number_length / group_length;
                if (number_length % group_length == 0) && (number_of_groups >= 2) {
                    for x in 10_i64.pow(group_length - 1)..10_i64.pow(group_length) {
                        let mut number = 0;
                        for k in 0..number_of_groups {
                            number += x * 10_i64.pow(k * group_length);
                        }
                        if (number >= start && number <= end) {
                            invalid_ids.insert(number);
                        }
                    }
                }
            }
        }

        ans +=  invalid_ids.into_iter().sum::<i64>();
    }
    println!("{}",ans);
}

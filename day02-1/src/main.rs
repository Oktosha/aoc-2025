use std::cmp::{max, min};
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let ranges = data.split(",");
    let mut ans = 0_i64;
    for range in ranges {
        let mut a = range.split("-");
        let start = a.next().unwrap().parse::<i64>().unwrap();
        let end = a.next().unwrap().parse::<i64>().unwrap();
        println!("{:?}-{:?}", start, end);
        for halflen in 1..10 {
            if halflen * 2 >= start.to_string().len() && halflen * 2 <= end.to_string().len() {
                let bottom = 10_i64.pow((2 * halflen - 1) as u32);
                let top = 10_i64.pow((2 * halflen) as u32) - 1;
                let l = max(bottom, start);
                let r = min(top, end);
                println!("{:?} {:?}", l, r);
                let l_first_half = l / 10_i64.pow(halflen as u32);
                let l_second_half = l % 10_i64.pow(halflen as u32);
                let r_first_half = r / 10_i64.pow(halflen as u32);
                let r_second_half = r % 10_i64.pow(halflen as u32);
                if (l_first_half == r_first_half) {
                    if (l_second_half <= l_first_half && r_second_half >= l_first_half) {
                        let invalid_id = l_first_half * (10_i64.pow(halflen as u32) + 1);
                        println!("{}", invalid_id);
                        ans += invalid_id;
                    }
                } else {
                    if (l_second_half <= l_first_half) {
                        let invalid_id = l_first_half * (10_i64.pow(halflen as u32) + 1);
                        println!("{}", invalid_id);
                        ans += invalid_id;
                    }
                    if (r_second_half >= r_first_half) {
                        let invalid_id = r_first_half * (10_i64.pow(halflen as u32) + 1);
                        println!("{}", invalid_id);
                        ans += invalid_id;
                    }
                    let a = l_first_half;
                    let b = r_first_half - 1;
                    let sum =
                        (b * (b + 1) / 2 - a * (a + 1) / 2) * (10_i64.pow(halflen as u32) + 1);
                    ans += sum;
                }
            }
        }
        println!();
    }
    println!("{}", ans);
}

use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt")
        .expect("File not found, please download input.txt from adventofcode.com");
    let mut grid = std::io::BufReader::new(file).lines().map(|l| l.unwrap().chars().collect()).collect::<Vec<Vec<char>>>();
    let mut ans = 0;
    let mut can_remove = true;
    while can_remove
    {
        can_remove = false;
        for i in 0_i32..grid.len() as i32 {
            for j in 0_i32..grid[i as usize].len() as i32 {
                if grid[i as usize][j as usize] == '@' {
                    let mut count = 0;
                    for (x, y) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                        if (y + i >= 0) && (y + i < grid.len() as i32) && (x + j >= 0) && (x + j < grid[i as usize].len() as i32) {
                            if (grid[(y + i) as usize][(x + j) as usize] == '@') || (grid[(y + i) as usize][(x + j) as usize] == 'x') {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        ans += 1;
                        grid[i as usize][j as usize] = 'x';
                    }
                }
            }
        }
        for i in 0_i32..grid.len() as i32 {
            for j in 0_i32..grid[i as usize].len() as i32 {
                if grid[i as usize][j as usize] == 'x' {
                    grid[i as usize][j as usize] = '.';
                    can_remove = true;
                }
            }
        }
    }
    println!("{}", ans);
}


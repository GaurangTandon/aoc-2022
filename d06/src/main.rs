use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/6.txt").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    println!("{}", line.len());
    let mut i = 0;
    let mut seen = [0 as u32; 256];
    let mut q = VecDeque::with_capacity(14);

    for ch in line.chars() {
        i += 1;

        q.push_back(ch);
        seen[ch as usize] += 1;

        let mut s = 0;
        for x in seen {
            if x > 1 {
                s = 0;
                break;
            } else {
                s += x;
            }
        }
        if s == 14 {
            println!("Found: {}", i);
            break;
        }

        if q.len() >= 14 {
            seen[q.pop_front().unwrap() as usize] -= 1;
        }

    }
}

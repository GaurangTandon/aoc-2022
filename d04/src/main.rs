use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/4.txt").unwrap();
    let reader = BufReader::new(file);
    let mut answer: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (p1, p2) = line.split_once(',').unwrap();
        let (x, y) = (p1.clone().split_once('-').unwrap(), p2.split_once('-').unwrap());
        let ((a, b), (c, d)) = ((x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()), (y.0.parse::<u32>().unwrap(), y.1.parse::<u32>().unwrap()));
        // answer += if (c <= a && b <= d) || (a <= c && d <= b) { 1 } else { 0 };
        answer += if b < c || d < a { 0 } else { 1 };
    }
    println!("{}", answer);
}

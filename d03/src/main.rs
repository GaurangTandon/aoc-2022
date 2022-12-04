use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn from_num(num: usize) -> char {
    return char::from_u32(num as u32 - 1 + if num <= 26 { 'a' } else { 'A' } as u32 - if num <= 26 { 0 } else { 26 }).unwrap();
}

fn to_num(ch: char) -> usize {
    (ch as u32 + 1 - if ch.is_lowercase() { 'a' } else { 'A' } as u32) as usize + (if ch.is_lowercase() { 0 } else { 26 })
}

fn get_common(lines: &[String]) -> char {
    let mut seen = [0; 52];
    for line in lines {
        let mut seen2 = [false; 52];
        for ch in line.chars() {
            let ind = to_num(ch) - 1;
            if !seen2[ind] {
                seen[ind] += 1;
                seen2[ind] = true;
            }
        }
    }
    for (index, val) in seen.into_iter().enumerate() {
        if val == 3 {
            let ans = from_num(index + 1);
            return ans;
        }
    }
    return 'A';
}

fn main() {
    let file = File::open("../inputs/3.txt").unwrap();
    let reader = BufReader::new(file);
    let answer: usize = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().chunks(3).map(|line| get_common(line)).map(|ch| to_num(ch)).sum();

    println!("{}", answer);
}

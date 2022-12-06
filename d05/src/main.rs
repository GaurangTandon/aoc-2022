use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    const COUNT: usize = 9;
    let mut stacks: [Vec<char>; COUNT] = Default::default();

    let file = File::open("../inputs/5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut liter = reader.lines();
    let mut lines: Vec<String> = liter.by_ref().take(8).map(|line| line.unwrap()).collect();
    lines.reverse();
    for l in lines {
        let b = l.as_bytes();
        for j in 0..COUNT {
            let k = 4 * j + 1;
            let c = b[k] as char;
            if c != ' ' {
                stacks[j].push(c);
            }
        }
    }

    liter.next();
    liter.next();

    for line in liter {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split(' ').collect();
        let count = parts[1].parse::<usize>().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        let mut data: Vec<char> = vec![];
        for _ in 0..count {
            data.push(stacks[from - 1].pop().unwrap());
        }
        for i in (0..count).rev() {
            stacks[to - 1].push(data[i]);
        }
    }

    for i in 0..COUNT {
        print!("{}", stacks[i].pop().unwrap());
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

#[derive(Clone)]
struct Holder {
    val: u8
}

#[derive(PartialEq, Clone, Hash, Eq)]
enum Players {
    Them, Us
}

fn mapper(val: Players) -> char { 
    if val == Players::Them { 'A' } else { 'X' }
}

impl Holder {
    fn from(val: char, repr: Players) -> Self {
        let sub = mapper(repr);
        let num = val as u8 - sub as u8;
        Holder { val: num }
    }

    fn wins_over(self: &Self) -> Self {
        Holder { val: if self.val == 0 { 2 } else { self.val - 1 }  }
    }

    fn loses_to(self: &Self) -> Self {
        Holder { val: if self.val == 2 { 0 } else { self.val + 1 }  }
    }

    fn score(self: &Self) -> u8 {
        self.val + 1
    }
}

fn day01() {
    let file = File::open("../inputs/2.txt").expect("Opened correctly");
    let reader = BufReader::new(file);
    let winning_combos = vec![('A', 'Z'), ('B', 'X'), ('C', 'Y')];
    let mut sum: u32 = 0;
    for (index, line) in reader.lines().map(|line| line.unwrap()).enumerate() {
        let (other, mine) = line.split_once(' ').unwrap();
        let (oc, mc) = (other.chars().next().unwrap(), mine.chars().next().unwrap());
        let s1 = if (oc as u32 - 'A' as u32) == (mc as u32 - 'X' as u32) {
            3
        } else if winning_combos.contains(&(oc, mc)) {
            0
        } else {
            6
        };
        let s2 = ((mc as u32) - ('X' as u32)) + 1;
        sum += s1 + s2;
        if index < 10 {
            println!("{} {} {} {} {}", s1, s2, oc, mc, sum);
        }
    }
    println!("{}", sum);
}

fn main() {
    let file = File::open("../inputs/2.txt").expect("Opened correctly");
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let (other, mine) = line.split_once(' ').unwrap();
        let (oc, mc2) = (Holder::from(other.chars().next().unwrap(), Players::Them), mine.chars().next().unwrap());
        let mc = if mc2 == 'X' {
            oc.wins_over()
        } else if mc2 == 'Y' {
            oc.clone()
        } else {
            oc.loses_to()
        };
        let s1 = if mc2 == 'X' { 0 } else if mc2 == 'Y' { 3 } else { 6 };
        let s2 = mc.score();
        sum += s1 + s2 as u32;
    }
    println!("{}", sum);
}

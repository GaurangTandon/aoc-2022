use std::{
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    collections::BinaryHeap
};

fn main() {
    let file = File::open("../inputs/1.txt").expect("File read correctly");
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut heap = BinaryHeap::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        if line == "" {
            heap.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    println!("{}", heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap());
}

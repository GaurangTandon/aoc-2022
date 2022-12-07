use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/7.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    // root directory has no size
    let mut current_dir_sizes: Vec<u32> = vec![0];
    let mut answer_part_1 = 0;
    let mut answer_part_2 = 0;

    // given in question
    const PART_1_THRESHOLD: u32 = 100000;
    const SPACE_REQUIRED: u32 = 30000000;
    const SPACE_TOTAL: u32 = 70000000;

    // by running this code once, we can get this value and paste it here
    // we can also just loop through the file twice, but this is simpler
    const SPACE_USED: u32 = 48008081;
    // this is how much more space we need to free up
    const SPACE_TO_FREE_UP: u32 = SPACE_REQUIRED - (SPACE_TOTAL - SPACE_USED);

    for line in lines {
        let l = line.unwrap();
        if l.starts_with("$ ls") {
            // ignore, as these directories will be visited in depth first order below
        } else if l.starts_with("$ cd") {
            // get the dirname
            let mut sit = l.split(' ');
            sit.next();
            sit.next();
            let dirname = sit.next().unwrap();

            if dirname == "/" {
                // only happens at the top of the input
            } else if dirname == ".." {
                // traverse up, update answers, and accumulate into parent

                let latest_value = *current_dir_sizes.last().unwrap();
                if latest_value <= PART_1_THRESHOLD {
                    answer_part_1 += latest_value;
                }
                if latest_value >= SPACE_TO_FREE_UP
                    && (answer_part_2 == 0 || answer_part_2 > latest_value)
                {
                    answer_part_2 = latest_value;
                }
                current_dir_sizes.pop();
                // accumulate the dir size into the parent directory
                *current_dir_sizes.last_mut().unwrap() += latest_value;
            } else {
                // new empty directory created
                current_dir_sizes.push(0);
            }
        } else if l.starts_with("dir") {
            // ignore as it will be visited by cd anyway
        } else {
            // it's a file, add its size to the current directory
            let num = l.split_once(' ').unwrap().0;
            let size = num.parse::<u32>().unwrap();
            let last_item = current_dir_sizes.last_mut().unwrap();
            *last_item += size;
        }
    }

    // traverse back up and accumulate the remaining directories into root
    while current_dir_sizes.len() > 1 {
        let latest_value = *current_dir_sizes.last().unwrap();
        if latest_value >= SPACE_TO_FREE_UP && (answer_part_2 == 0 || answer_part_2 > latest_value)
        {
            answer_part_2 = latest_value;
        }
        current_dir_sizes.pop();
        // accumulate the dir size into the parent directory
        *current_dir_sizes.last_mut().unwrap() += latest_value;
    }

    let root_size = *current_dir_sizes.last().unwrap();
    println!("{}", root_size);
    println!("{}", answer_part_1);
    println!("{}", answer_part_2);
}

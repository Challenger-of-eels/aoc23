use std::collections::{HashMap, HashSet};
use crate::common::*;

/* CRAP */
pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut sum:i32 = 0;

    for l in read_lines(input_file_path)? {
        let line = l?;
        let (tail, head) = consume_to(&line, ": ");
        let mut hash:HashSet<u32> = HashSet::new();
        let (nums, wins) = consume_to(&tail, "| ");
        for win_str in wins.split(" ") {
            if win_str.len() == 0 {
                continue;
            }
            let win = win_str.parse::<u32>().unwrap();
            hash.insert(win);
        }
        let mut power = 0;
        for num_str in nums.split(" ") {
            if num_str.len() == 0 {
                continue;
            }
            let num = num_str.parse::<u32>().unwrap();
            if hash.contains(&num) {
                if power == 0 {
                    power = 1;
                } else {
                    power = power * 2;
                }
            }
        }
        sum += power;
    }
    println!("{sum}");
    Ok(())
}


/* CLEAN */
pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut sum:u32 = 0;
    let mut card_index:usize = 0;
    let mut copies:Vec<u32> = vec![];

    for l in read_lines(input_file_path)? {
        let line = l?;
        let (head, tail) = &line.split_once(": ").unwrap();
        let (wins, nums) = &tail.split_once(" | ").unwrap();
        let hash:HashSet<&str> = HashSet::from_iter(wins.split(" ").filter(|v| !v.is_empty()));
        let power = nums.split(" ")
            .filter(|v| hash.contains(v))
            .count();

        let max_affected = card_index + power + 1;
        if copies.len() < max_affected {
            copies.resize(max_affected, 1);
        }
        let multiplier = copies[card_index];
        for i in card_index + 1..max_affected {
            copies[i] += multiplier;
        }
        card_index += 1;
        sum += multiplier;
    }
    println!("{:?}", copies);
    println!("{sum}");
    Ok(())
}
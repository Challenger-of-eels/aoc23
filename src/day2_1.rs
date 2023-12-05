use std::collections::HashMap;
use std::fs;
use crate::common::*;

pub fn main(input_file_path: &str) {
    let mut sum:u32 = 0;
    let max_counts:HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)].iter().cloned().collect();

    for line in fs::read_to_string(input_file_path).unwrap().lines() {
        let tail = consume(line, "Game ");
        let (game_str, game_id) = consume_u32(tail, ": ");
        if is_game_possible(game_str, &max_counts) {
            sum += game_id;
        }
    }
    println!("{sum}");
}

fn is_game_possible(game_str: &str, max_counts:&HashMap<&str, u32>) -> bool {
    for draw_str in game_str.split("; ") {
        for group in draw_str.split(", ") {
            let (color, count) = consume_u32(group, " ");
            if max_counts.get(color).unwrap_or(&0) < &count {
                return false;
            }
        }
    }
    return true;
}
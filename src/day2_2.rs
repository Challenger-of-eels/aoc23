use std::collections::HashMap;
use std::fs;

pub fn main(input_file_path: &str) {
    let mut sum:u32 = 0;

    for line in fs::read_to_string(input_file_path).unwrap().lines() {
        let l = line.len();
        let mut tail = consume(line, "Game ");
        let (game_str, game_id) = consume_u32_to(tail, ": ");
        sum += get_game_value(game_str);
    }
    println!("{sum}");
}

fn get_game_value(game_str: &str) -> u32 {
    let mut max_counts:HashMap<&str, u32> = HashMap::new();

    for draw_str in game_str.split("; ") {
        for group in draw_str.split(", ") {
            let (color, count) = consume_u32_to(group, " ");
            if max_counts.get(color).unwrap_or(&0) < &count {
                max_counts.insert(color, count);
            }
        }
    }

    let mut power = 1;
    for colorCount in max_counts.values() {
        power *= colorCount;
    }
    return power;
}

fn consume<'a>(line:&'a str, start:&str) -> &'a str {
    debug_assert!(line.starts_with(start));
    return &line[start.len()..];
}

fn consume_to<'a>(line:&'a str, up_to:&str) -> (&'a str, &'a str) {
    if let Some(pos) = line.find(up_to) {
        return (&line[pos + up_to.len()..], &line[0..pos]);
    }
    panic!("no up_to string found");
}
fn consume_u32_to<'a>(line:&'a str, up_to:&str) -> (&'a str, u32) {
    let (end, value_str) = consume_to(line, up_to);
    if let Ok(value)= value_str.parse::<u32>() {
        return (end, value);
    }
    panic!("not an u32");
}
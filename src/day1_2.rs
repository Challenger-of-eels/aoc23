use std::collections::HashMap;
use std::fs;

pub fn main(input_file_path: &str) {
    let mut map:HashMap<String, u8> = HashMap::new();

    for digit in 0..=9 {
        let word: String = ((digit + b'0') as char).to_string();
        map.insert(word.clone(), digit);
    }
    let words = "one two three four five six seven eight nine".split_whitespace().collect::<Vec<_>>();
    for i in 0..words.len() {
        map.insert(words[i].to_string(), (i + 1) as u8);
    }

    let mut sum:u32 = 0;
    for line in fs::read_to_string(input_file_path).unwrap().lines() {
        let l = line.len();
        let mut first_value:u8 = 0;
        let mut first_pos:usize = l;
        let mut last_value:u8 = 0;
        let mut last_pos:usize = 0;
        for (word, digit) in &map {
            /* CRAP, why not to find ALL ENTRIES EVERYWHERE */
            if let Some(first) = line.find(word.as_str()) {
                if first_pos > first {
                    first_pos = first;
                    first_value = *digit;
                }
            }
            if let Some(last) = line.rfind(word.as_str()) {
                if last_pos < last + word.len() {
                    last_pos = last + word.len();
                    last_value = *digit;
                }
            }
        }
        sum += (first_value * 10 + last_value) as u32;

    }
    println!("{sum}");
}
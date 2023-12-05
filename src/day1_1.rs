use std::fs;

pub fn main(input_file_path: &str) {
    let mut sum:u32 = 0;
    for line in fs::read_to_string(input_file_path).unwrap().lines() {
        let forward = Box::new(line.as_bytes().iter());
        let backward = Box::new(line.as_bytes().iter().rev());
        sum += first_digit(forward) * 10 + first_digit(backward);

    }
    println!("{sum}");
}

fn first_digit(bytes: Box<dyn Iterator<Item = &u8> + '_>) -> u32 {
    for char in bytes {
        if char.is_ascii_digit() {
            return (char - b'0') as u32;
        }
    }
    return 0;
}
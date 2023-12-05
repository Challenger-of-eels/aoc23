use std::{io::{self, BufRead}, fs::File, path::Path};


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let buf = read_lines(filename)?;
    let mut lines:Vec<String> = vec![];
    for l in buf {
        lines.push(l?);
    }
    return Ok(lines)
}


pub fn consume<'a>(line:&'a str, start:&str) -> &'a str {
    debug_assert!(line.starts_with(start));
    return &line[start.len()..];
}

/* use "".split_once(..) please */
pub fn consume_to<'a>(line:&'a str, up_to:&str) -> (&'a str, &'a str) {
    if let Some(pos) = line.find(up_to) {
        return (&line[pos + up_to.len()..], &line[0..pos]);
    }
    panic!("no up_to string found");
}

pub fn consume_u32<'a>(line:&'a str, up_to:&str) -> (&'a str, u32) {
    let (value_str, end) = consume_to(line, up_to);
    if let Ok(value)= value_str.parse::<u32>() {
        return (end, value);
    }
    panic!("not an u32");
}
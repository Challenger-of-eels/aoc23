use crate::common::*;

/* much better then day3_1, also cleaned up afterwards */

struct Number {
    value:i32,
    x:i32,
    w:i32,
}
struct Symbol {
    value:u8,
    x:i32,
}
struct Row {
    symbols:Vec<Symbol>,
    numbers:Vec<Number>,
}

impl Row {
    fn new() -> Row {
        return Row {
            symbols: vec![],
            numbers: vec![],
        }
    }
}

pub fn main(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut sum:i32 = 0;

    let mut rows:Vec<Row> = vec![Row::new(), Row::new(), Row::new()];

    for l in read_lines(input_file_path)? {
        let tmp = rows.pop().unwrap();
        rows.insert(0, tmp);
        parse_row(&mut rows[0], l?);
        collect_sum_from_row(1, &rows, &mut sum);
    }
    rows.pop();
    collect_sum_from_row(0, &rows, &mut sum);
    println!("{sum}");
    Ok(())
}

fn collect_sum_from_row(index:usize, mut rows:&Vec<Row>, sum:&mut i32) {
    for gear in &rows[index].symbols {
        let mut neib:Vec<i32> = vec![];
        for row in rows {
            collect_neib_numbers_from_row(gear.x, &mut neib, &row.numbers);
        }
        if neib.len() == 2 {
            *sum += neib[0] * neib[1];
        }
    }
}

fn collect_neib_numbers_from_row(x:i32, result:&mut Vec<i32>, all:&Vec<Number>) {
    for number in all {
        if number.x - 1 <= x && number.x + number.w >= x {
            result.push(number.value);
        }
    }
}

fn parse_row(row:&mut Row, line:String) {
    row.symbols.clear();
    row.numbers.clear();

    let mut x:i32 = 0;
    let mut value:i32 = 0;
    let mut w:i32 = 0;
    let l = line.len() - 1;
    for char in line.chars() {
        let is_digit = char.is_ascii_digit();
        if is_digit {
            value = value * 10 + (char as u8 - b'0') as i32;
            w += 1;
        } else if char == '*' {
            row.symbols.push(Symbol {
                x:x,
                value:char as u8,
            });
        }
        if w > 0 && (!is_digit || (x as usize == l)) {
            row.numbers.push(Number {
                value,
                x: x - w + (is_digit as i32),
                w
            });
            value = 0;
            w = 0;
        }
        x += 1;
    }
}
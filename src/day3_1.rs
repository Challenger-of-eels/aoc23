use std::collections::HashMap;
use crate::common::*;

/* CRAP */
struct Number {
    value:i32,
    x:i32,
    w:i32,
    valid:bool
}

impl Number {
    fn copy(&self) -> Number {
        return Number {
            value: self.value,
            x: self.x,
            w: self.w,
            valid: self.valid,
        }
    }
}

fn clear_number(n: &mut Number) {
    n.value = 0;
    n.x = 0;
    n.w = 0;
    n.valid = false;
}

pub fn main(input_file_path: &str) -> Result<(), std::io::Error> {

    let mut lastRowNumbers:Vec<Number> = vec![];
    let mut rowNumbers:Vec<Number> = vec![];
    let mut lastRow:String = String::from("");
    let mut lastSymbol:bool = false;
    let mut lastNumber:Number = Number {
        value: 0,
        x: 0,
        w: 0,
        valid:false,
    };

    let mut sum = 0;

    for l in read_lines(input_file_path)? {
        let line = l?;
        let mut x:i32 = 0;
        for char in line.chars() {
            match char {
                '.' => {
                    lastSymbol = false;
                    if lastNumber.w > 0 {
                        if lastRow.len() > 0 {
                            match lastRow.as_bytes()[x as usize] {
                                b'.' => {},
                                d if (d as u8).is_ascii_digit() => {},
                                _ => lastNumber.valid = true
                            }
                        }
                        if lastNumber.valid {
                            sum += lastNumber.value;
                            println!("{}", lastNumber.value);
                        } else {
                            rowNumbers.push(lastNumber.copy());
                        }
                        clear_number(&mut lastNumber);
                    }
                },
                d if d.is_ascii_digit() => {
                    let digit = d as i32 - b'0' as i32;
                    if lastNumber.w == 0 {
                        lastNumber.x = x;
                        lastNumber.w = 1;
                        lastNumber.value = digit;
                        lastNumber.valid = lastSymbol;

                        if lastRow.len() > 0 && x > 0 {
                            match lastRow.as_bytes()[(x - 1) as usize] {
                                b'.' => {},
                                d if (d as u8).is_ascii_digit() => {},
                                _ => lastNumber.valid = true
                            }
                        }
                    } else {
                        lastNumber.w += 1;
                        lastNumber.value = lastNumber.value * 10 + digit;
                    }
                    if lastRow.len() > 0 {
                        match lastRow.as_bytes()[x as usize] {
                            b'.' => {},
                            d if (d as u8).is_ascii_digit() => {},
                            _ => lastNumber.valid = true
                        }
                    }
                },
                _ => {
                    lastSymbol = true;
                    if lastNumber.w > 0 {
                        sum += lastNumber.value;
                        println!("{}", lastNumber.value);
                        clear_number(&mut lastNumber);
                    }
                    if lastRow.len() > 0 {
                        for i in 0..lastRowNumbers.len() {
                            if let mut number = &mut lastRowNumbers[i] {
                                if !number.valid {
                                    if number.x - 1 <= x && number.x + number.w >= x {
                                        number.valid = true;
                                        sum += number.value;
                                        println!("{} {} {} {}", number.value, x, number.x, number.w);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            x += 1;
        }

        if lastNumber.w > 0 {
            if lastNumber.valid {
                sum += lastNumber.value;
                println!("{}", lastNumber.value);
            } else {
                rowNumbers.push(lastNumber.copy());
            }
            clear_number(&mut lastNumber);
        }

        lastRowNumbers = rowNumbers;
        rowNumbers = vec![];
        lastRow = line.clone();
    }
    println!("{sum}");
    Ok(())
}
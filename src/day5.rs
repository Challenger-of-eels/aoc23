use std::collections::{HashMap, HashSet};
use crate::common::*;

pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut gen = 1;
    let mut lines = read_lines(input_file_path)?;
    let mut values = lines.next().unwrap()?.strip_prefix("seeds: ").unwrap().split(" ").map(|s|s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut lastModified:Vec<u64> = values.iter().map(|_|0).collect::<_>();
    for l in lines {
        let line = l?;
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            println!("{}", line);
            gen = gen + 1;
            continue;
        }

        let m:Vec<u64> = line.split(" ").map(|s|s.parse::<u64>().unwrap()).collect();
        let mut i = 0;
        for value in &mut values {
            if *value >= m[1] && *value < m[1] + m[2]  && lastModified[i] < gen {
                let diff:i64 = m[0] as i64 - m[1] as i64;
                *value = (*value as i64 + diff) as u64;
                lastModified[i] = gen;
            }
            i = i + 1;
        }
        println!("{}", values.iter().map(|v|v.to_string()).collect::<Vec<_>>().join(" "));
    }
    let min = values.iter().min().unwrap();
    println!("{min}");
    Ok(())
}

struct Span {
    min: i64,
    max: i64
}

impl Span {
    fn new(min:i64, max:i64) -> Span {
        return Span { min, max };
    }
    fn with_offset(min:i64, max:i64, offset:i64) -> Span {
        return Span { min: min + offset, max: max + offset };
    }
}

pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut gen = 1;
    let mut lines = read_lines(input_file_path)?;
    let mut spans:Vec<Span> = vec![];
    let mut new_spans:Vec<Span> = vec![];
    let mut values:Vec<i64> =
        lines.next().unwrap()?
        .strip_prefix("seeds: ").unwrap()
        .split(" ")
        .map(|s|s.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut i = 0;
    while i < values.len() {
        spans.push(Span {
            min:values[i],
            max:values[i] + values[i + 1] - 1
        });
        i = i + 2;
    }
    println!("{},{}",spans.len(),values.len());
    // let mut lastModified:Vec<u64> = values.iter().map(|_|0).collect::<_>();
    for l in lines {
        let line = l?;
        if line.is_empty() {
            //this method removes all elements from arg and places them in self
            spans.append(&mut new_spans);
            println!("{}", spans.iter().map(|s| s.min.to_string() + "~" + &s.max.to_string()).collect::<Vec<_>>().join(" "));
            continue;
        }
        if !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
    //
        let m:Vec<i64> = line.split(" ").map(|s|s.parse::<i64>().unwrap()).collect();
        let dst = m[0];
        let min = m[1];
        let max = min + m[2] - 1;
        let d = dst - min;
        for i in (0..spans.len()).rev() {
            println!("{} {} {}", line, dst, d);
            let span = &spans[i];
            let span_min = span.min;
            let span_max = span.max;

            if min >= span_min && min <= span_max {
                if min > span_min {
                    spans.push(Span::new(span_min, min - 1));
                }
                if max < span_max {
                    spans.push(Span::new(max + 1, span_max));
                    new_spans.push(Span::with_offset(min, max, d));
                } else {
                    new_spans.push(Span::with_offset(min, span_max, d));
                }
                spans.remove(i);
                // span.max = min;
            } else if min < span_min && max >= span_min {
                if max < span_max {
                    spans.push(Span::new(max + 1, span_max));
                    new_spans.push(Span::with_offset(span_min, max, d));
                    // span.min = max
                } else {
                    new_spans.push(Span::with_offset(span_min, span_max, d));

                }
                spans.remove(i);
            }
        }
    //     for i in
    //     for value in &mut values {
    //         if *value >= m[1] && *value < m[1] + m[2]  && lastModified[i] < gen {
    //             let diff:i64 = m[0] as i64 - m[1] as i64;
    //             *value = (*value as i64 + diff) as u64;
    //             lastModified[i] = gen;
    //         }
    //         i = i + 1;
    //     }
    //     println!("{}", values.iter().map(|v|v.to_string()).collect::<Vec<_>>().join(" "));
    }

    spans.append(&mut new_spans);
    println!("{}", spans.iter().map(|s| s.min.to_string() + "~" + &s.max.to_string()).collect::<Vec<_>>().join(" "));
    let min = spans.iter().reduce(|a, b| {
        if a.min < b.min {
            a
        } else {
            b
        }
    }).unwrap().min;
    println!("{min}");
    Ok(())
}
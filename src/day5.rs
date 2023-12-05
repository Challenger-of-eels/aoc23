use std::cmp;
use crate::common::*;

pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut gen = 1;

    let mut lines = read_lines(input_file_path)?;

    let mut values:Vec<i64> =
        lines.next().unwrap()?
        .strip_prefix("seeds: ").unwrap()
        .split(" ")
        .map(|s|s.parse::<i64>().unwrap())
        .collect();

    let mut last_modified:Vec<i64> = vec![0; values.len()];

    for l in lines {
        let line = l?;
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            gen = gen + 1;
            continue;
        }

        let m:Vec<i64> =
            line.split(" ")
            .map(|s|s.parse::<i64>().unwrap()).collect();

        let mut i = 0;
        for value in &mut values {
            if *value >= m[1] && *value < m[1] + m[2] && last_modified[i] < gen {
                let diff:i64 = m[0] as i64 - m[1] as i64;
                *value = *value + diff;
                last_modified[i] = gen;
            }
            i = i + 1;
        }
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
}

pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
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
        spans.push(Span::new(values[i], values[i] + values[i + 1] - 1));
        i = i + 2;
    }

    for l in lines {
        let line = l?;
        if line.is_empty() {
            //this method removes all elements from arg and places them in self
            spans.append(&mut new_spans);
            continue;
        }
        if !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let m:Vec<i64> =
            line.split(" ")
            .map(|s|s.parse::<i64>().unwrap())
            .collect();
        let dst = m[0];
        let min = m[1];
        let max = min + m[2] - 1;
        let offset = dst - min;
        for i in (0..spans.len()).rev() {
            let span = &spans[i];
            // hail rust for excess code but safety
            let span_min = span.min;
            let span_max = span.max;

            fn try_add_span(min:i64, max:i64, some_spans:&mut Vec<Span>, offset:i64) {
                if max >= min {
                    some_spans.push(Span::new(min + offset, max + offset));
                }
            }

            let moved_min:i64 = cmp::max(span_min, min);
            let moved_max:i64 = cmp::min(span_max, max);
            if moved_min <= moved_max {
                try_add_span(moved_min, moved_max, &mut new_spans, offset);
                try_add_span(moved_max + 1, span_max, &mut spans, 0);
                try_add_span(span_min, moved_min - 1, &mut spans, 0);
                spans.remove(i);
            }
        }
    }
    spans.append(&mut new_spans);

    let min = spans.iter().min_by_key(|a|a.min).unwrap().min;
    println!("{min}");
    Ok(())
}
use std::{fs::File, io::{BufReader, BufRead}};

pub fn main() {
    p1("input11.txt");
    p2("input11.txt");
}

#[derive(Debug)]
struct V2 {
    x:i32,
    y:i32,
}

fn p1(file:&str) -> Result<(),anyhow::Error> {
    calc(file, 2)
}

fn p2(file:&str) -> Result<(),anyhow::Error> {
    calc(file, 1000000)
}


fn calc(file:&str, expansion:i32) -> Result<(),anyhow::Error> {
    let (galaxies, empty_space_x, empty_space_y) = parse(file)?;

    let mut sum: i64 = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let a = &galaxies[i];
            let b = &galaxies[j];
            sum += (
                (a.x - b.x).abs() + (a.y - b.y).abs() + (
                    (empty_space_x[a.x as usize] - empty_space_x[b.x as usize]).abs() + 
                    (empty_space_y[a.y as usize] - empty_space_y[b.y as usize]).abs()
                ) * (expansion - 1)
            ) as i64;
        }
    }

    dbg!(sum);
    Ok(())
}

fn parse(file:&str) -> Result<(Vec<V2>, Vec<i32>, Vec<i32>),anyhow::Error> {
    let mut galaxies:Vec<V2> = vec![];
    let mut empty_space_x:Vec<i32> = vec![];
    let mut empty_space_y:Vec<i32> = vec![];

    let mut y = 0;
    for line in BufReader::new(File::open(file)?).lines() {
        let l = line?;

        empty_space_y.push(1);
        if empty_space_x.len() == 0 {
            empty_space_x.resize(l.len(), 1);
        }

        let mut x = 0;
        for byte in l.as_bytes() {
            match byte {
                b'#' => {
                    galaxies.push(V2 {x, y});
                    empty_space_x[x as usize] = 0;
                    empty_space_y[y as usize] = 0;
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    fn sum_to_index(a:&mut Vec<i32>) {
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
            a[i] = sum;
        }
    }

    sum_to_index(&mut empty_space_x);
    sum_to_index(&mut empty_space_y);

    Ok((galaxies, empty_space_x, empty_space_y))
}



use std::{fs::File, io::{BufReader, BufRead}, path::Path};
use std::time::Instant;

type ParseResult = Vec<Vec<bool>>;

pub fn main() {
    let file = Path::new(file!()).join("../input.txt");

    let input: Vec<Vec<bool>> = parse(&file).unwrap();
    
    measure(p1, &input);//34889 0.7ms

    measure(p2, &input);//34224
    //brute force  4.8ms
    //count_ones   0.8ms
}

fn p1(input:&ParseResult) {
    solve(input, false,);
}

fn p2(input:&ParseResult) {
    solve(input, true);
}

fn solve(input:&ParseResult, look_up_for_smudge:bool) {
    let mut sum = 0;
    let mut first_not_empty = 0;
    for i in 0..input.len() {
        if input[i].len() == 0 {
            sum += solve_frame(&input[first_not_empty..i], look_up_for_smudge);
            first_not_empty = i + 1;
        }
    }
    sum += solve_frame(&input[first_not_empty..input.len()], look_up_for_smudge);
    dbg!(sum);
}

fn solve_frame(frame:&[Vec<bool>], look_up_for_smudge:bool) -> i32 {
    // represent frames in binary form aka 001101010
    // we could parse() straight to i32 but who knows what is in the part 2
    let mut h:Vec<i32> = frame.iter().map(|v| {
        let mut r = 0;
        for b in v {
            r = r << 1;
            if *b {
                r |= 1;
            }
        }
        r
    }).collect::<_>();

    let mut v = vec![0; frame[0].len() as usize];
    for i in 0..frame.len() {
        for j in 0..v.len() {
            v[j] <<= 1;
            if frame[i][j] {
                v[j] |= 1;
            }
        }
    }

    if look_up_for_smudge {
        return find_potential_mirror(&h) * 100 + find_potential_mirror(&v);
            
        //p2 brute force solution, needs (original)
        //return brute_force_exhaustive_search(frame, &mut v, &mut h, &original);
    } else {
        let original:(i32, i32) = (find_mirror(&h, 0), find_mirror(&v, 0));
        return original.0 * 100 + original.1;
    }
}


fn find_mirror(v:&Vec<i32>, result_to_skip:i32) -> i32 {
    'outer: for i in 0..v.len() - 1 {
        if i as i32 + 1 == result_to_skip {
            continue;
        }
        let min = (v.len() - i - 1).min(i + 1);
        for j in 0..min {
            if v[i + 1 + j] != v[i - j] {
                continue 'outer;
            }
        }
        return i as i32 + 1;
    }
    return 0;
}


// count_ones solution, was 10 times faster to print, 10 times longer to find
fn find_potential_mirror(v:&Vec<i32>) -> i32 {
    for i in 0..v.len() - 1 {
        let mut error:u32 = 0;
        let min = (v.len() - i - 1).min(i + 1);
        for j in 0..min {
            let xor = v[i + 1 + j] ^ v[i - j];
            error += xor.count_ones();
            if error > 1 { break; } // premature optimization
        }
        if error == 1 {
            return i as i32 + 1;
        }
    }
    return 0;
}


//first solution, unused
fn brute_force_exhaustive_search(frame:&[Vec<bool>], v:&mut Vec<i32>, h:&mut Vec<i32>, original:&(i32,i32)) -> i32 {
    let mut smudge_h = 1;
    for i in 0..frame[0].len() {
        let mut smudge_v = 1;
        for j in 0..frame.len() {
            v[i] ^= smudge_v;
            h[j] ^= smudge_h;
            let attempt = find_mirror(&h, original.0) * 100 + find_mirror(&v, original.1);
            v[i] ^= smudge_v;
            h[j] ^= smudge_h;
            if attempt != 0 {
                return attempt;
            }
            smudge_v <<= 1;
        }
        smudge_h <<= 1;
    }
    dbg!("Not found!");
    return 0;
}



fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    Ok(lines.map(|l| {
        let line = l.unwrap();
        line.chars().map(|b| match b {
            '#' => true,
            _ => false
        }).collect::<Vec<bool>>()
    }).collect::<_>())
}

fn measure(p:fn(input:&ParseResult), input:&ParseResult) {
    let now = Instant::now();
    p(input);
    let t = now.elapsed().as_micros() as f64 / 1e6;
    println!("Elapsed {}s", t);
}
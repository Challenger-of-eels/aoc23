use crate::common::read_lines;

enum Direction {
    Start,
    End
}

pub fn p1(file:&str) -> Result<(),std::io::Error> {
    return solve(file, Direction::End);
}

pub fn p2(file:&str) -> Result<(),std::io::Error> {
    return solve(file, Direction::Start);
}

fn solve(file:&str, direction:Direction) -> Result<(),std::io::Error> {
    let lines = read_lines(file);

    let mut sum = 0;

    for l in lines? {
        let line = l?;
        let input_seq: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<_>();

        sum += rec(input_seq, &direction);
    }

    println!("{}", sum);

    Ok(())
}

fn rec(seq:Vec<i32>, direction:&Direction) -> i32 {
    // println!("{:?}", seq);

    let mut dif:Vec<i32> = Vec::with_capacity(seq.len() - 1);
    let mut all_zeroes = true;

    for i in 0..seq.len() - 1 {
        let v = seq[i + 1] - seq[i];
        dif.push(v);
        all_zeroes &= v == 0
    }

    if all_zeroes {
        return seq[0];
    }

    let diff_result = rec(dif, &direction);
    return match direction {
        Direction::Start =>
            seq[0] - diff_result,
        Direction::End =>
            seq[seq.len() - 1] + diff_result
    }
}
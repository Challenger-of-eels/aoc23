use crate::common::*;


pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;


    // let separator:Regex = Regex::new(r".*\w+").unwrap();
    let mut values:Vec<Vec<i32>> = vec![];
    for line in lines {
        values.push(line?.split_once(":").unwrap().1
            .split(" ")
            .filter(|l| l.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<_>()
        );
    }

    let mut result:i32 = 1;
    for (&time, &distance) in values[0].iter().zip(values[1].iter()) {
        let mut sum = 0;
        for t in 0..time {
            if t * (time - t) > distance {
                sum += 1;
            }
        }
        result *= sum;
    }
    println!("{:?}", values);
    println!("{result}");
    Ok(())
}


pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;


    // let separator:Regex = Regex::new(r".*\w+").unwrap();
    let mut values:Vec<i64> = vec![];
    for line in lines {
        values.push(line?.split_once(":").unwrap().1
            .split(" ")
            .filter(|l| l.len() > 0).collect::<Vec<&str>>().join("")
            .parse::<i64>().unwrap()
        );
    }
    println!("{:?}", values);

    let mut result:i64 = 1;
    let (time, distance) = (values[0], values[1]);
    let mut sum = 0;
    for t in 0..time {
        if t * (time - t) > distance {
            sum += 1;
        }
    }
    result *= sum;
    println!("{result}");
    Ok(())
}
use crate::common::*;


pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;

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


// let's solve ax^2 + bx + c
// AKA t * (time - t) == distance
// AKA -t*t + t * time - distance == 0
pub fn p2_smart_ass(input_file_path: &str) -> Result<(), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;

    let mut values:Vec<f64> = vec![];
    for line in lines {
        values.push(
            line?.replace(" ", "")
            .split_once(":").unwrap().1
            .parse::<f64>().unwrap()
        );
    }
    println!("{:?}", values);

    let (time, distance) = (values[0], values[1]);

    fn solve(a:f64, b:f64, c:f64) -> [f64;2] {
        let mut d_sqrt = (b * b - 4.0 * a * c).sqrt();
        if a < 0.0 {
            d_sqrt = -d_sqrt;// now roots are sorted
        }
        return [
            (-b - d_sqrt) / 2.0 / a,
            (-b + d_sqrt) / 2.0 / a
        ];
    }
    let roots = solve (-1.0, time, -distance);
    let result:f64 = (roots[1].floor() - roots[0].ceil() + 1.0).min(time);
    println!("{result}");
    Ok(())
}
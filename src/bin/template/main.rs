use std::{fs::File, io::{BufReader, BufRead}, path::Path};
use std::time::Instant;

type ParseResult = Vec<String>/*TODO*/;

pub fn main() {
    let file = Path::new(file!()).join("../input.txt");

    let input:ParseResult = parse(&file).unwrap();
    dbg!(&input);/*TODO*/

    measure(p1, &input);
    measure(p2, &input);
}

// #[derive(PartialEq, Clone, Copy)]

fn p1(input:&ParseResult) {

}


fn p2(input:&ParseResult) {

}


fn solve() {

}




fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    Ok(lines.map(|l| {
        let line = l.unwrap();
        line
    }).collect::<_>())
}

fn measure(p:fn(input:&ParseResult), input:&ParseResult) {
    let now = Instant::now();
    p(input);
    let t = now.elapsed().as_micros() as f64 / 1e6;
    println!("Elapsed {}s", t);
}
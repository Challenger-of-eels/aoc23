use std::borrow::Cow;
use std::collections::HashMap;
use crate::common::*;

fn tuple_to_string(tuple:(&str, &str)) -> (String, String) {
    let sa = tuple.0.to_string();
    let sb = tuple.1.to_string();
    return (sa, sb);
}

pub fn parse(input_file_path: &str) -> Result< (Vec<u8>, HashMap<String,(String,String)>), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;

    let b = lines.next().unwrap().unwrap().clone();
    let commands: Vec<u8> = b.as_bytes().to_vec();
    let mut map:HashMap<String,(String,String)> = HashMap::new();

    lines.next();

    for line in lines {
        let l = line.unwrap();
        let (node, left_right) = tuple_to_string(l.split_once(" = (").unwrap());
        let (left, right) = tuple_to_string(left_right.trim_end_matches(")").split_once(", ").unwrap());
        map.insert(node, (left, right));
    }

    Ok((commands, map))
}
pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    let (commands, map) = parse(input_file_path).unwrap();

    let commands_length = commands.len();
    let mut current_node = &"AAA".to_string();
    let mut c_index:usize = 0;
    while current_node != "ZZZ" {
        if commands[c_index % commands_length] == b'L' {
            current_node = &map[current_node].0;
        } else {
            current_node = &map[current_node].1;
        }
        c_index += 1;
    }
    println!("{}", c_index);
    Ok(())
}

fn last_byte_is(s: &&String, l:u8) -> bool {
    return s.as_bytes()[s.len() - 1] == l;
}


pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
    let (commands, map) = parse(input_file_path).unwrap();

    let commands_length = commands.len();
    let mut nodes:Vec<String> = map.keys()
        .map(|a|a.to_string())
        .filter(|key| last_byte_is(&key, b'A'))
        .collect::<_>();

    //not part of solution
    //spend a lot of time figuring out how data structured
    examine_data_p2(commands.clone(), map.clone(), nodes.clone());

    let mut multipliers:Vec<f64> = vec![];

    for loop_node in nodes {
        let mut node = loop_node;
        let mut z_index: usize = 0;
        while !last_byte_is(&&node, b'Z') {
            if commands[z_index % commands_length] == b'L' {
                node = map[&node].0.to_string();
            } else {
                node = map[&node].1.to_string();
            }
            z_index += 1;
        }
        multipliers.push(z_index as f64);
    }

    //least_common_multiplier
    //naive bruteforce
    //better use rust lib or Stein's algorithm (i google'd it afterwards)
    fn least_common_multiplier(a:f64, b:f64) -> f64 {
        let mut result = 1.0;
        let mut a = a;
        let mut b = b;
        let mut i = (a*b).sqrt().floor();
        while i > 1.0 {
            while a % i == 0.0 && b % i == 0.0 {
                a /= i;
                b /= i;
                result *= i;
            }
            i = i - 1.0;
        }
        return a * b * result
    }

    // Rust pain
    // if use .iter() and not into_iter() then least_common_multiplier should be
    // fn least_common_multiplier<'a>(a:&'a f64, b:&'a f64) -> &'a f64
    // it's impossible or something
    // but into_iter consumes vector and it can't be used after that line

    // common delimiter 271.0, now you now
    let result = multipliers.into_iter().reduce(least_common_multiplier).unwrap() as i64;

    println!("{:?}", result); //21165830176709

    Ok(())
}







fn examine_data_p2(commands:Vec<u8>, map:HashMap<String,(String,String)>, nodes:Vec<String>) {
    let commands_length = commands.len();
    let mut multipliers:Vec<f64> = vec![];
    let mut cicles:Vec<f64> = vec![];

    for loop_node in nodes {
        let mut node= loop_node;
        let mut z_index:usize = 0;
        let mut sample:Vec<String> = vec![];
        let mut sample2:Vec<String> = vec![];
        while !last_byte_is(&&node, b'Z') {
            if sample.len() < 10 {
                sample.push(node.to_string());
            }
            if commands[z_index % commands_length] == b'L' {
                node = map[&node].0.to_string();
            } else {
                node = map[&node].1.to_string();
            }
            z_index += 1;
        }
        multipliers.push(z_index as f64);
        let z_node = node.clone();
        let mut c_index = 0;
        while !last_byte_is(&&node, b'Z') || c_index == 0 {
            if sample2.len() < 10 {
                sample2.push(node.to_string());
            }
            let (l,r) = &map[&node];
            if commands[z_index % commands_length] == b'L' {
                node = l.to_string();
            } else {
                node = r.to_string();
            }
            c_index += 1;
            z_index += 1;
        }
        cicles.push(c_index as f64);

        // first 1A->1Z and second looped 1Z->1Z not equal but quickly merges 1A->1Z->1Z->1Z->1Z

        // first 10 nodes of every first loop (from A to first Z)
        println!("{:?}", sample);   // eg ["HJA", "XMG", "XKD", "GBJ", "DJT", "CQM", "NBN", "FFD", "JPQ", "MKC"]
        // first 10 nodes of every second loop (from first Z to second Z in one route)
        println!("{:?}", sample2);  // eg ["RSZ", "SVM", "XKD", "GBJ", "DJT", "CQM", "NBN", "FFD", "JPQ", "MKC"]
    }

    // first 1A->1Z and looped 1Z->1Z equal-length for every *A and *Z pair
    // that means that least_common_multiplier strategy is valid
    // if there was 1A->2A->1Z->2Z loop or 1A->1Z->2A->2Z loop it would be even harder
    println!("{:?}", multipliers);
    println!("{:?}", cicles);
}




//first crap attempt. Tried naive bruteforce, just to be sure. had no chance, too long
pub fn p2_too_long(input_file_path: &str) -> Result<(), std::io::Error> {
    let (commands, map) = parse(input_file_path).unwrap();

    let commands_length = commands.len();
    let mut nodes:Vec<&String> = map.keys().filter(|key| last_byte_is(key, b'A')).collect::<_>();
    let mut c_index:usize = 0;
    while !nodes.iter().all(|key| last_byte_is(&key, b'Z')) {
        for i in 0..nodes.len() {
            if commands[c_index % commands_length] == b'L' {
                nodes[i] = &map[nodes[i]].0;
            } else {
                nodes[i] = &map[nodes[i]].1;
            }
        }
        c_index += 1;
    }
    println!("{}", c_index);
    Ok(())
}
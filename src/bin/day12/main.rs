use std::{fs::File, io::{BufReader, BufRead}};
use std::time::Instant;

pub fn main() {
    //test();

    p1("src/bin/day12/input.txt");//6852

    // have a lot of known room for optimization but not much in terms of O(N)
    let now = Instant::now();
    p2("src/bin/day12/input.txt");//8475948826693, 2.5s on battery 1.3s with power supply
    dbg!(now.elapsed());
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    On, Off, Unknown
}

impl Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::On => ".",
            Tile::Off => "#",
            Tile::Unknown => "?",
        }.to_string()
    }

    fn from_char(c:char) -> Tile {
        match c {
            '.' => Tile::On,
            '#' => Tile::Off,
            _ => Tile::Unknown,
        }
    }

    fn vec_from_string(s:&str) -> Vec<Tile> {
        return s.chars().map(Tile::from_char).collect::<_>()
    }

    fn vec_to_string(tiles:&Vec<Tile>) -> String {
        return tiles.iter().map(Tile::to_string).collect::<Vec<_>>().join("")
    }
}


fn test() {
    fn assert_line(tiles_str:&str, groups:&[i32], expected:i64) {
        let tiles =  &Tile::vec_from_string(&tiles_str.to_string());
        let result = solve(tiles, groups, (0,tiles.len()));
        assert!(result == expected);
        println!("{} {:?} {}", Tile::vec_to_string(tiles), groups, result);
    }
    assert_line("?##???#?", &[2,1], 1);
    assert_line("???????#?#", &[1,1], 1);
    assert_line("??????#?#?#", &[1,1,1], 1);
}


fn p1(file:&str)->Result<(),anyhow::Error> {
    let input = parse(file)?;
    let mut sum = 0;
    for (tiles, groups) in input {
        sum += solve(&tiles, &groups, (0,tiles.len()));
    }
    dbg!(sum);
    Ok(())
}


fn p2(file:&str)->Result<(),anyhow::Error> {
    let input = parse(file)?;
    let mut sum:i64 = 0;
    
    let mut line_num = 0;
    for (mut tiles, mut groups) in input {
        tiles = std::iter::repeat(tiles).take(5)
            .collect::<Vec<_>>()
            .join(&Tile::Unknown);
        groups = groups.repeat(5);

        let value = solve(&tiles, &groups, (0, tiles.len()));
        sum += value;
        
        line_num += 1;
        // println!(" line {} = {}", line_num, value);
    }
    dbg!(sum);
    Ok(())
}


fn solve(tiles:&[Tile], groups:&[i32], offset:(usize,usize)) -> i64 {
    let mut result:i64 = 0;
    let tiles_len = tiles.len();
    let groups_len = groups.len();

    let mid = groups_len / 2;
    let mid_size = groups[mid] as usize;

    let left = &groups[..mid];
    let right = &groups[mid + 1..];
    let min_left = offset.0 + left.iter().sum::<i32>() as usize + mid;
    let min_right = offset.1 - (right.iter().sum::<i32>() as usize + mid_size + groups_len - mid - 1);
    

    // println!(" {:?} {:?}", groups, min_left..min_right);
    
    let mut last_chance = false;
    'outer: for i in min_left..=min_right {
        if last_chance {
            break;
        } else if mid == 0 && tiles[i] == Tile::Off {
            //no # can be found before it
            last_chance = true;
        }


        if i > 0 && tiles[i - 1] == Tile::Off {
            continue;
        }
        if i + mid_size < tiles_len && tiles[i + mid_size] == Tile::Off {
            continue;
        }
        if tiles[i..i+mid_size].iter().any(|t| *t == Tile::On) {
            continue;
        }
        let rightmost = mid == groups_len - 1;
        if rightmost {
            //no # can be found after it
            for j in i + mid_size + 1..offset.1 {
                if tiles[j] == Tile::Off {
                    continue 'outer;
                }
            }
        }

        let count_left:i64;
        if mid == 0 {
            count_left = 1;
        } else {
            count_left = solve(tiles, left, (offset.0, i - 1));
        }

        let count_right:i64;
        if rightmost {
            count_right = 1;
        } else {
            count_right = solve(tiles, right, (i + mid_size + 1, offset.1));
        }

        result += count_left * count_right;
    }
    return result;
}


fn parse(file:&str)->Result<Vec<(Vec<Tile>,Vec<i32>)>,anyhow::Error> {
    Ok(BufReader::new(File::open(file)?).lines().map(|l| {
        let line = l.unwrap();
        let (map,nums) = line.split_once(" ").unwrap();

        let groups = nums.split(",").map(|n|
                n.parse::<i32>().unwrap()
            ).collect::<_>();

        let tiles:Vec<Tile> = Tile::vec_from_string(map);

        (tiles, groups) 
    }).collect::<_>())
}
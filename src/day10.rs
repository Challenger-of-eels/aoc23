use std::collections::HashMap;
use std::io::Error;
use std::ops::Index;
use crate::common::read_lines;

pub fn main() {
    p1("input10.txt");
    p2("input10.txt");
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[derive(Debug)]
enum Direction {
    Top, Right, Bottom, Left
}

impl Direction {
    fn to_string(self) -> String {
        return match self {
            Direction::Top => "Top",
            Direction::Right => "Right",
            Direction::Bottom => "Bottom",
            Direction::Left => "Left",
        }.to_string()
    }
    
    fn opposite(self) -> Direction {
        return match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    fn get_offset(self) -> V2 {
        return match self {
            Direction::Right => V2 {x:1, y:0},
            Direction::Left => V2 {x:-1, y:0},
            Direction::Top => V2 {x:0, y:-1},
            Direction::Bottom => V2 {x:0, y:1},
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::Top, Direction::Right, Direction::Bottom, Direction::Left]
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Tile {
    TL,TR,BL,BR,
    V,H,None,Start
}

struct TileInfo {
    char:u8,
    inputs:Vec<Direction>
}

struct V2 {
    x:i32,
    y:i32,
}

struct Map<'a> {
    start:V2,
    tiles:Vec<Vec<&'a Tile>>
}

pub fn p1(file:&str)->Result<(),Error> {
    let (tile_config,tile_by_char) = make_tile_config();
    let map = parse(file, &tile_by_char)?;

    let mut result = 0;

    for start_direction in Direction::all() {
        let length = try_find_loop_length(&map, &start_direction, &tile_config, None);
        if length > 0 {
            result = length / 2;
            break;
        }
    }

    println!("{}", result);

    Ok(())
}



pub fn p2(file:&str)->Result<(),Error> {
    let (tile_config,tile_by_char) = make_tile_config();
    let map = parse(file, &tile_by_char)?;

    let mut loop_map:Vec<Vec<Tile>> = vec![vec![Tile::None; map.tiles[0].len()]; map.tiles.len()];

    fn clear_loop_map(loop_map:&mut Vec<Vec<Tile>>) {
        for y in 0..loop_map.len() {
            for x in 0..loop_map[y].len() {
                loop_map[y][x] = Tile::None;
            }
        }
    }

    fn print_loop_map(loop_map:&Vec<Vec<Tile>>, tile_config:&HashMap<Tile, TileInfo>) {
        println!("{}", loop_map.iter().map(|v|
                v.iter().map(|t|
                    (tile_config[t].char as char).to_string()
                ).collect::<Vec<String>>().join("")
            ).collect::<Vec<String>>().join("\n")
        );
    }

    for start_direction in Direction::all() {
        let length = try_find_loop_length(&map, &start_direction, &tile_config, Some(&mut loop_map));
        if length > 0 {
            break;
        } else {
            clear_loop_map(&mut loop_map);
        }
    }

    // can't handle arbitrary Start positions, only some cases
    //     need fix in try_find_loop_length, set S as valid part of the loop

    let mut sum = 0;
    for y in 0..loop_map.len() {
        let mut inside:bool = false;
        let mut line_cross_start:Tile = Tile::None;
        for x in 0..loop_map[y].len() {
            let mut char_inside:bool = false;
            let tile = loop_map[y][x];
            match tile {
                Tile::Start => { panic!("can't handle") },
                Tile::V => inside = !inside,
                Tile::BR | Tile::TR => {
                    inside = !inside;
                    line_cross_start = tile;
                },
                Tile::H => { },
                Tile::BL => {
                    if line_cross_start != Tile::TR {
                        inside = !inside;
                    }
                    line_cross_start = Tile::None;
                },
                Tile::TL => {
                    if line_cross_start != Tile::BR {
                        inside = !inside;
                    }
                    line_cross_start = Tile::None;
                },
                _ => {
                    if inside {
                        sum += 1;
                        char_inside = true;
                    }
                }
            }

            //comment to print loop, uncomment to print "inside" as `S` and loop as `|`
            loop_map[y as usize][x as usize] = if char_inside {
                Tile::Start
            } else if tile != Tile::None {
                Tile::V
            } else {
                tile
            }
        }
    }

    print_loop_map(&loop_map, &tile_config);
    println!("{}", sum);

    Ok(())
}

fn try_find_loop_length(map:&Map, start_direction:&Direction, tile_config:&HashMap<Tile, TileInfo>, mut loop_map:Option<&mut Vec<Vec<Tile>>>) -> i32 {
    let empty_row:Vec<&Tile> = vec![];
    let mut direction = start_direction;
    let mut distance = 0;
    let mut pos = V2 {x:map.start.x, y:map.start.y};
    loop {
        let step = direction.get_offset();
        pos.x += step.x;
        pos.y += step.y;
        distance += 1;

        if pos.x == map.start.x && pos.y == map.start.y {
            return distance;
        }

        // peek "." if tile is outside of the map
        let tile = map.tiles
            .get(pos.y as usize).unwrap_or_else(||&empty_row)
            .get(pos.x as usize).unwrap_or_else(||&&Tile::None);

        let tile_info:&TileInfo = &tile_config[tile];

        let enter_direction = &direction.opposite();
        let can_enter = tile_info.inputs.contains(enter_direction);
        let other_input = tile_info.inputs.iter().find(|d|d != &enter_direction);

        // print step by step
        // println!("start:{} `{}` {},{} from {} > {}",
        //          start_direction.to_string(),
        //          tile_info.char as char, pos.x, pos.y,
        //          enter_direction.to_string(),
        //          if can_enter {
        //             other_input.unwrap().to_string()
        //          } else {
        //              "STOP".to_string()
        //          }
        // );

        if can_enter {
            direction = other_input.unwrap();

            // draw loop in loop_map if provided
            if let Some(ref mut m) = loop_map {
                m[pos.y as usize][pos.x as usize] = **tile;
            }
        } else {
            return 0;
        }
    }
}

fn make_tile_config() -> (HashMap<Tile, TileInfo>, HashMap<u8, Tile>) {
    let mut tile_config:HashMap<Tile, TileInfo> = HashMap::new();
    tile_config.insert(Tile::TR, TileInfo { char:b'L', inputs:vec![Direction::Top, Direction::Right] });
    tile_config.insert(Tile::TL, TileInfo { char:b'J', inputs:vec![Direction::Top, Direction::Left] });
    tile_config.insert(Tile::BR, TileInfo { char:b'F', inputs:vec![Direction::Bottom, Direction::Right] });
    tile_config.insert(Tile::BL, TileInfo { char:b'7', inputs:vec![Direction::Bottom, Direction::Left] });
    tile_config.insert(Tile::H, TileInfo { char:b'-', inputs:vec![Direction::Right, Direction::Left] });
    tile_config.insert(Tile::V, TileInfo { char:b'|', inputs:vec![Direction::Bottom, Direction::Top] });
    tile_config.insert(Tile::None, TileInfo { char:b'.', inputs:vec![] });
    tile_config.insert(Tile::Start, TileInfo { char:b'S', inputs:vec![] });


    let mut tile_by_char:HashMap<u8, Tile> = HashMap::new();
    for (tile, tile_info) in &tile_config {
        tile_by_char.insert(tile_info.char, *tile);
    }
    return (tile_config, tile_by_char);
}


fn parse<'a>(file:&str, tile_by_char:&'a HashMap<u8, Tile>) -> Result<Map<'a>,Error> {
    let mut start:V2 = V2 {x:0, y:0};
    let lines = read_lines(file);

    let mut tiles:Vec<Vec<&Tile>> = vec![];
    let mut y:i32 = 0;
    for l in lines? {
        tiles.push(vec![]);
        let line = l?;
        let mut x:i32 = 0;
        for char in line.as_bytes() {
            match char {
                b'S' => start = V2 {x, y},
                _ => {}
            }
            tiles[y as usize].push(&tile_by_char[char]);
            x += 1;
        }
        y += 1;
    }
    Ok(Map {start, tiles})
}
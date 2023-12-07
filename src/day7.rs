use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use crate::common::*;

//#[derive(Debug)] for verbose printlns
struct Hand {
    cards:String,//for debug
    values_of_cards:Vec<u8>,
    combination:[i32; 2],//two numbers is enough to encode hand type
    score:i32,
}

// for compact printlns
impl Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.cards)
            .field(&self.values_of_cards)
            .field(&self.combination)
            .field(&self.score)
            .finish()
    }
}

fn compare_hands(a:&Hand, b:&Hand) -> Ordering {
    for (ca, cb) in a.combination.iter().zip(b.combination.iter()) {
        if ca.cmp(cb) != Ordering::Equal {
            return ca.cmp(cb);
        }
    }
    for (va, vb) in a.values_of_cards.iter().zip(b.values_of_cards.iter()) {
        if va.cmp(vb) != Ordering::Equal {
            return va.cmp(vb);
        }
    }
    panic!("5 equal cards! error");
}
pub fn p1(input_file_path: &str) -> Result<(), std::io::Error> {
    return common_solution(input_file_path,"AKQJT98765432" ,false );
}

pub fn p2(input_file_path: &str) -> Result<(), std::io::Error> {
    return common_solution(input_file_path,"AKQT98765432J" ,true );
}

pub fn common_solution(input_file_path: &str, card_order: &str, joker_rule:bool) -> Result<(), std::io::Error> {
    let mut lines = read_lines(input_file_path)?;
    let mut hands:Vec<Hand> = vec![];
    for line in lines {
        let l = line.unwrap();
        let (cards, score_str) = l.split_once(" ").unwrap();
        let score = score_str.parse::<i32>().unwrap();

        let cards = cards.to_string();
        let values_of_cards =
            cards.as_bytes().iter()
                .map(|c|card_order.len() as u8 - card_order.find(*c as char).unwrap() as u8)
                .collect::<_>();

        let mut array_of_cards:Vec<u8> = cards.as_bytes().iter().map(|a|*a).collect::<_>();
        array_of_cards.sort();

        const DECK_SIZE:i32 = 5;
        let mut combination:[i32; DECK_SIZE as usize] = [0; DECK_SIZE as usize];
        let mut j = 0;
        let mut joker = 0;
        for i in 0..array_of_cards.len() {
            if joker_rule && array_of_cards[i] == b'J' {
                joker += 1;
            } else if i == 0 {
                continue;
            } else if array_of_cards[i] == array_of_cards[i - 1] {
                combination[j] += 1
            } else {
                j += 1;
                if j >= combination.len() {
                    break;
                }
            }
        }
        combination.sort();
        combination.reverse();//for example [2,1,0,0,0] means 3,2 - full house
        if joker_rule {
            combination[0] = (DECK_SIZE - 1).min(combination[0] + joker);
        }
        let combination:[i32; 2] = combination[0..2].try_into().unwrap();

        hands.push(Hand {cards, values_of_cards, combination, score});
    }

    hands.sort_by(compare_hands);

    let mut sum= 0;
    for i in 0..hands.len() {
        sum += hands[i].score * (i as i32 + 1);
    }

    println!("{:?}", hands);
    println!("{:?}", sum);

    Ok(())
}
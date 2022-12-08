#![allow(unused_variables)]

use std::fs;

pub fn winner(player1: u32, player2: u32) -> bool {
    const ROCK: u32 = 1;
    const PAPER: u32 = 2;
    const SCISSORS: u32 = 3;
    match player2 {
        ROCK => {
            match player1 {
                SCISSORS => true,
                _ => false,
            }
        }
        PAPER => {
            match player1 {
                ROCK => true,
                _ => false,
            }
        }
        _ => {
            match player1 {
                PAPER => true,
                _ => false,
            }
        }
    }
}

pub fn parse_choice(s: &str) -> Result<u32, &str> {
    match s.chars().next().unwrap() {
        'A' => Ok(1), //rock
        'B' => Ok(2), //paper
        'C' => Ok(3), //scissors
        'X' => Ok(1), //rock
        'Y' => Ok(2), //paper
        'Z' => Ok(3), //scissors
        _ => Err("Cheating!"),
    }
}

pub fn strategy(player1: u32, outcome: u32) -> u32 {
    const LOSE: u32 = 1;
    const DRAW: u32 = 2;
    const ROCK: u32 = 1;
    const PAPER: u32 = 2;
    const SCISSORS: u32 = 3;
    match outcome {
        LOSE => {
            print!("Need to loose -->");
            match player1 {
                ROCK => SCISSORS,
                PAPER => ROCK,
                _ => PAPER,
            }
        }
        DRAW =>
            match player1 {
                ROCK => ROCK,
                PAPER => PAPER,
                _ => SCISSORS,
            }
        _ =>
            match player1 {
                ROCK => PAPER,
                PAPER => SCISSORS,
                _ => ROCK,
            }
    }
}

pub fn match_points(input: &Vec<&str>) -> u32 {
    let player1 = parse_choice(input[0]).unwrap();
    let player2 = parse_choice(input[1]).unwrap();
    let bonus: u32 = if player1 == player2 {
        3
    } else {
        if winner(player1, player2) { 6 } else { 0 }
    };
    bonus + player2
}

pub fn match_points2(input: &Vec<&str>) -> u32 {
    let player1 = parse_choice(input[0]).unwrap();
    let plan = parse_choice(input[1]).unwrap();
    let player2 = strategy(player1, plan);
    let bonus: u32 = if player1 == player2 {
        3
    } else {
        if winner(player1, player2) { 6 } else { 0 }
    };
    println!("bonus {}", bonus);
    bonus + player2
}

pub fn total_points(matches: &String) -> u32 {
    matches
        .lines()
        .map(|s| {
            let choices: Vec<&str> = s.split_ascii_whitespace().collect();
            match_points2(&choices)
        })
        .sum()
}

fn main() {
    let file_path = format!("{}/src/game.txt", env!("CARGO_MANIFEST_DIR"));
    let matches = &fs::read_to_string(&file_path).unwrap();
    println!("{:?}", total_points(matches))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score() {
        assert_eq!(8, match_points(&vec![&'A'.to_string()[..], &'Y'.to_string()[..]]));
        assert_eq!(1, match_points(&vec![&'B'.to_string()[..], &'X'.to_string()[..]]));
        assert_eq!(6, match_points(&vec![&'C'.to_string()[..], &'Z'.to_string()[..]]));
    }

    #[test]
    fn get_score_part2() {
        assert_eq!(4, match_points2(&vec![&'A'.to_string()[..], &'Y'.to_string()[..]])); // loose
        assert_eq!(1, match_points2(&vec![&'B'.to_string()[..], &'X'.to_string()[..]])); // draw
        assert_eq!(7, match_points2(&vec![&'C'.to_string()[..], &'Z'.to_string()[..]])); // win
    }

    #[test]
    fn simple() {
        let input = &String::from("\
        A Y
        B X
        C Z");
        assert_eq!(12, total_points(input));
    }
}
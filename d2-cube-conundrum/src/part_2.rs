#![allow(dead_code)]
use std::cmp;

use crate::{CubeSet, CubeGame};

fn process(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            process_line(line)
        })
        .sum()
}

fn process_line(line: &str) -> u32 {
    let game = line.parse::<CubeGame>().unwrap();
    let min_bag = game.sets.iter().fold(CubeSet{ red: 0, green: 0, blue: 0 }, |mut acc_set, curr_set| {
        acc_set.red = cmp::max(acc_set.red, curr_set.red);
        acc_set.green = cmp::max(acc_set.green, curr_set.green);
        acc_set.blue = cmp::max(acc_set.blue, curr_set.blue);

        acc_set
    });

    min_bag.red * min_bag.green * min_bag.blue
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_line_1() {
        let line_possible = "Game 11: 9 blue, 1 red, 6 green; 6 red, 1 green; 10 blue, 3 green, 6 red";

        assert_eq!(process_line(line_possible), 6u32 * 6u32 * 10u32);
    }

    #[test]
    fn test_line_2() {
        let line_possible = "Game 13: 3 red, 11 green, 18 blue; 11 green, 1 red, 3 blue; 12 blue, 5 red, 2 green; 16 blue, 8 red, 5 green; 8 red, 12 blue, 19 green; 17 blue, 4 green, 6 red";

        assert_eq!(process_line(line_possible), 8u32 * 19u32 * 18u32);
    }

    #[test]
    fn test_process() {
        let result = process(INPUT);
        println!("RESULT_2: {result}");
    }
}

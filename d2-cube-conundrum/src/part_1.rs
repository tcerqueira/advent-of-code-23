#![allow(dead_code)]
use crate::{CubeSet, CubeGame};


fn process(input: &str, bag: &CubeSet) -> u32 {
    input.lines()
        .map(|line| {
            process_line(line, bag)
        })
        .sum()
}

fn process_line(line: &str, bag: &CubeSet) -> u32 {
    let game = line.parse::<CubeGame>().unwrap();
    let impossible = game.sets.iter().any(|set| {
        set.red > bag.red || set.green > bag.green || set.blue > bag.blue
    });

    if impossible {
        return 0u32
    }
    game.id
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");
    const BAG: CubeSet = CubeSet { red: 12, green: 13, blue: 14 };

    #[test]
    fn test_line_possible() {
        let line_possible = "Game 11: 9 blue, 1 red, 6 green; 6 red, 1 green; 10 blue, 3 green, 6 red";

        assert_eq!(process_line(line_possible, &BAG), 11u32);
    }

    #[test]
    fn test_line_impossible() {
        let line_impossible = "Game 13: 3 red, 11 green, 18 blue; 11 green, 1 red, 3 blue; 12 blue, 5 red, 2 green; 16 blue, 8 red, 5 green; 8 red, 12 blue, 19 green; 17 blue, 4 green, 6 red";

        assert_eq!(process_line(line_impossible, &BAG), 0u32);
    }

    #[test]
    fn test_process() {
        let result = process(INPUT, &BAG);
        println!("RESULT_1: {result}");
    }
}

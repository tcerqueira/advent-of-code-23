use std::str::FromStr;

pub mod part_1;
pub mod part_2;

#[derive(Default)]
struct CubeGame {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32, 
}

impl FromStr for CubeSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = CubeSet::default();

        for entry in s.trim().split(", ") {
            let mut entry_iter = entry.split_whitespace();
            let count: u32 = entry_iter.next().unwrap().parse().unwrap();

            match entry_iter.next().unwrap() {
                "red" => cube_set.red = count,
                "green" => cube_set.green = count,
                "blue" => cube_set.blue = count,
                _ => panic!("unknown color"),
            }
        }

        Ok(cube_set)
    }
}

impl FromStr for CubeGame {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_game = CubeGame::default();

        let mut colon_iter = s.trim().split(':');

        let game_str = colon_iter.next().unwrap();
        let mut game_iter = game_str.split_whitespace();
        let _ = game_iter.next();
        cube_game.id = game_iter.next().unwrap().parse().unwrap();

        for cube_set in colon_iter.next().unwrap().trim().split("; ") {
            cube_game.sets.push(cube_set.parse().unwrap());
        }

        Ok(cube_game)
    }
}

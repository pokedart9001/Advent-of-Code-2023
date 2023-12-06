use std::{fs, io};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

struct GameSet {
    red: u32,
    blue: u32,
    green: u32
}

impl GameSet {
    fn new(game_set: &str) -> Self {
        let game_sets: Vec<&str> = game_set
            .split(",")
            .map(str::trim)
            .collect();
    
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in game_sets.iter().map(|set| set.split(" ").collect::<Vec<_>>()) {
            let (num, color) = (set[0].parse().unwrap(), set[1]);

            if color == "red" {
                red = num;
            } else if color == "blue" {
                blue = num;
            } else if color == "green" {
                green = num;
            }
        }
        Self { red, blue, green }
    }
    
    // fn is_possible(&self, max_red: u32, max_blue: u32, max_green: u32) -> bool {
    //     self.red <= max_red && self.blue <= max_blue && self.green <= max_green
    // }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

// fn is_possible(game: &[GameSet]) -> bool {
//     game.iter().all(|game_set| game_set.is_possible(12, 14, 13))
// }

fn fewest_possible(game: &[GameSet]) -> GameSet {
    let (red, blue, green) = game.iter()
        .fold((0, 0, 0), |acc, e| (
            u32::max(acc.0, e.red),
            u32::max(acc.1, e.blue),
            u32::max(acc.2, e.green)
        ));
    
    GameSet { red, blue, green }
}

fn main() -> io::Result<()> {
    let input = input_lines("resources/input.txt")?;
    
    let games: Vec<(usize, Vec<GameSet>)> = input.iter()
        .map(|line| line.split([':', ';']).map(str::trim).skip(1).collect())
        .map(|game: Vec<&str>| game.iter().map(|desc| GameSet::new(&desc)).collect())
        .enumerate()
        .map(|(i, game)| (i+1, game))
        .collect();

    // let possible_games_sum: usize = games.iter()
    //     .filter(|(_, game)| is_possible(&game))
    //     .map(|(i, _)| i)
    //     .sum();
    // dbg!(possible_games_sum);

    let minimum_games_sum: u32 = games.iter()
        .map(|(_, game)| fewest_possible(&game).power())
        .sum();
    dbg!(minimum_games_sum);
    
    Ok(())
}

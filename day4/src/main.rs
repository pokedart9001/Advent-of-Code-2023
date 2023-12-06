use std::{fs, io};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

struct ScratchCard {
    winning_numbers: Box<[u32]>,
    numbers_to_match: Box<[u32]>
}

impl ScratchCard {
    fn new(number_set: Vec<String>) -> Self {
        let winning_numbers: Box<[u32]> = number_set[0]
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let numbers_to_match: Box<[u32]> = number_set[1]
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        Self { winning_numbers, numbers_to_match }
    }

    // fn points(&self) -> u32 {
    //     self.numbers_to_match.iter().fold(0, |acc, num|
    //         match (acc, num) {
    //             (0, num) if self.winning_numbers.contains(num) => 1,
    //             (acc, num) if self.winning_numbers.contains(num) => acc * 2,
    //             _ => acc
    //         }
    //     )
    // }

    fn num_matches(&self) -> usize {
        self.numbers_to_match.iter().filter(|num| self.winning_numbers.contains(num)).count()
    }
}

fn num_total_cards(cards: &[ScratchCard]) -> usize {
    let mut counts: Vec<usize> = cards.iter().map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        for j in 1..=card.num_matches() {
            counts[i+j] += counts[i];
        }
    }

    counts.iter().sum()
}

fn main() -> io::Result<()> {
    let input = input_lines("resources/input.txt")?;

    let cards: Vec<ScratchCard> = input.iter()
        .map(|line| line.split([':', '|'])
            .skip(1)
            .map(|s| s.trim().replace("  ", " "))
            .collect::<Vec<_>>()
        )
        .map(ScratchCard::new)
        .collect();

    // let card_points_sum: u32 = cards.iter()
    //     .map(|card| card.points())
    //     .sum();

    // dbg!(card_points_sum);

    dbg!(num_total_cards(&cards));

    Ok(())
}

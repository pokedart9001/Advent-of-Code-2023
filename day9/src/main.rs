use std::{fs, io};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

fn prediction(history: &Vec<i32>) -> i32 {
    let mut sequences = vec![history.to_vec()];
    while sequences.last().unwrap().iter().any(|x| *x != 0) {
        sequences.push(sequences.last().unwrap().windows(2).map(|nums| nums[1] - nums[0]).collect());
    }

    // sequences.iter().rev().fold(0, |acc, e| acc + e.last().unwrap())
    sequences.iter().rev().fold(0, |acc, e| e.first().unwrap() - acc)
}

fn main() -> io::Result<()> {
    let input = input_lines("resources/input.txt")?;
    
    let histories: Vec<Vec<i32>> = input.iter()
        .map(|line| line.split(" ").filter_map(|s| s.parse().ok()).collect())
        .collect();

    let prediction_sum: i32 = histories.iter().map(prediction).sum();
    dbg!(prediction_sum);

    Ok(())
}

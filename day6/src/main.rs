use std::{fs, io};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

struct Race {
    max_time: u64,
    record_distance: u64
}

impl Race {
    fn possible_distances(&self) -> Vec<u64> {
        (0..=self.max_time).map(|hold| hold * (self.max_time - hold)).collect()
    }

    fn num_ways_to_win(&self) -> usize {
        self.possible_distances().iter().filter(|dist| **dist > self.record_distance).count()
    }
}

// fn get_times_and_distances(lines: &[String]) -> Vec<Vec<u64>> {
//     lines.iter()
//         .flat_map(|line| line
//             .split(":")
//             .skip(1)
//             .map(|s| s.split_whitespace().filter_map(|s| s.parse().ok()).collect())
//             .collect::<Vec<_>>()
//         )
//         .collect()
// }

fn main() -> io::Result<()> {
    let input = input_lines("resources/input.txt")?;
    
    // let times_and_distances: Vec<Vec<u64>> = get_times_and_distances(&input);

    // let num_ways_to_win_product: usize = times_and_distances[0].iter().copied().zip(times_and_distances[1].iter().copied())
    //     .map(|(max_time, record_distance)| Race { max_time, record_distance }.num_ways_to_win())
    //     .product();

    // dbg!(num_ways_to_win_product);

    let time_and_distance: Vec<u64> = input.iter()
        .flat_map(|line| line
            .split(":")
            .skip(1)
            .filter_map(|s| s.replace(" ", "").parse().ok())
            .collect::<Vec<_>>()
        )
        .collect();

    let (max_time, record_distance) = (time_and_distance[0], time_and_distance[1]);
    let num_ways_to_win = Race { max_time, record_distance }.num_ways_to_win();

    dbg!(num_ways_to_win);

    Ok(())
}

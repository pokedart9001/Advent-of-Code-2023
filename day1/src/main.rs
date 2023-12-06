use std::{fs, io};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

fn first_and_last_occurance(digit_str: &str, line: &str) -> Option<(usize, usize)> {
    let first = line.find(digit_str);
    let last = line.rfind(digit_str);

    Some((first?, last?))
}

fn digit_replacements(line: &str) -> (Option<(usize, &str)>, Option<(usize, &str)>) {
    let digits = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    let replacements = digits
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, s)| (i, *s, first_and_last_occurance(s, line)))
        .filter_map(|(i, s, occurance)| Some((i, s, occurance?)));

    let first_replacement = replacements.clone()
        .min_by(|(_, _, (first1, _)), (_, _, (first2, _))| first1.cmp(first2))
        .map(|(i, s, _)| (i, s));
    let last_replacement = replacements
        .max_by(|(_, _, (_, last1)), (_, _, (_, last2))| last1.cmp(last2))
        .map(|(i, s, _)| (i, s));

    (first_replacement, last_replacement)
}

fn get_calibration_value(line: &str) -> u32 {
    let (first_replacement, last_replacement) = digit_replacements(line);

    let first_digit = match first_replacement {
        Some((i, s)) => line.replace(s, &i.to_string()),
        None => line.to_string()
    }
    .chars().find_map(|c| c.to_digit(10)).expect("Should have at least one digit");
    
    let last_digit = match last_replacement {
        Some((i, s)) => line.replace(s, &i.to_string()),
        None => line.to_string()
    }
    .chars().filter_map(|c| c.to_digit(10)).last().expect("Should have at least one digit");

    10 * first_digit + last_digit
}

fn main() -> io::Result<()> {
    let calibration_value_sum: u32 = input_lines("resources/input.txt")?
        .iter()
        .map(String::as_str)
        .map(get_calibration_value)
        .sum();

    dbg!(calibration_value_sum);

    Ok(())
}

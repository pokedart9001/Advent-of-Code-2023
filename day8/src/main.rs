use std::{fs, io, collections::HashMap};
use reikna::factor::lcm_all;

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

enum Instruction {
    Left,
    Right
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!()
        }
    }
}

fn nodes_from_info(nodes_info: Vec<Vec<&str>>) -> HashMap<String, (String, String)> {
    let nodes_iter = nodes_info.iter()
        .filter_map(|parts| Some((parts[0], parts[1][1..parts[1].len()-1].split_once(",")?)));
    HashMap::from_iter(nodes_iter.map(|(node, (left, right))| (node.to_owned(), (left.to_owned(), right.trim().to_owned()))))
}

fn follow_instructions(nodes: HashMap<String, (String, String)>, instructions: impl Iterator<Item = Instruction> + Clone, start: &str) -> Option<usize> {
    let mut current_node = start.to_owned();
    for (turn_num, turn) in instructions.enumerate() {
        current_node = match turn {
            Instruction::Left => nodes[&current_node].0.clone(),
            Instruction::Right => nodes[&current_node].1.clone()
        };
        
        if current_node.ends_with("Z") {
            return Some(turn_num + 1);
        }
    }
    None
}

fn main() -> io::Result<()> {
    let input = input_lines("resources/input.txt")?;

    let instructions = input[0].chars().map(Instruction::from).cycle();
    let nodes_info = input.iter()
        .skip(2)
        .map(|line| line.split("=").map(str::trim).collect())
        .collect();
    
    let nodes = nodes_from_info(nodes_info);
    let starting_nodes: Vec<String> = nodes.keys().filter_map(|node| if node.ends_with("A") { Some(node.to_owned()) } else { None }).collect();

    // let num_steps = follow_instructions(nodes, instructions, "AAA").unwrap();
    // dbg!(num_steps);

    let path_lengths: Vec<u64> = starting_nodes.iter()
        .filter_map(|node| follow_instructions(nodes.clone(), instructions.clone(), &node).map(|x| x as u64))
        .collect();

    dbg!(lcm_all(&path_lengths));

    Ok(())
}

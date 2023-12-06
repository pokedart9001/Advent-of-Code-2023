#![feature(iter_array_chunks)]

use std::{fs, io, ops::Range};

fn input_groups(filename: &str) -> io::Result<Vec<Vec<String>>> {
    fs::read_to_string(filename).map(|s| s.replace("\r\n", "\n").split("\n\n").map(|s| s.lines().map(ToString::to_string).collect()).collect())
}

struct ConversionRange {
    dest_range_start: u64,
    source_range_start: u64,
    range_len: u64
}

impl ConversionRange {
    fn new(desc: &str) -> Self {
        let vals: Vec<&str> = desc.split(" ").collect();
        let (dest_range_start, source_range_start, range_len) = (vals[0].parse().unwrap(), vals[1].parse().unwrap(), vals[2].parse().unwrap());

        Self { dest_range_start, source_range_start, range_len }
    }
    
    fn source_range(&self) -> Range<u64> {
        self.source_range_start..(self.source_range_start + self.range_len)
    }

    fn source_to_dest(&self, seed: u64) -> u64 {
        match seed {
            source if self.source_range().contains(&source) => source + self.dest_range_start - self.source_range_start,
            _ => seed
        }
    }
}

struct ConversionMap {
    ranges: Box<[ConversionRange]>
}

impl ConversionMap {
    fn convert(&self, seed: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.source_range().contains(&seed) {
                return range.source_to_dest(seed);
            }
        }
        seed
    }
}

fn get_seeds(seed_str: &str) -> Vec<u64> {
    seed_str
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() -> io::Result<()> {
    let input = input_groups("resources/input.txt")?;

    let seeds = get_seeds(&input[0][0]);
    let seeds: Vec<u64> = seeds.into_iter()
        .array_chunks::<2>()
        .flat_map(|[start, len]| (start..(start + len)).collect::<Vec<_>>())
        .collect();

    let maps: Vec<ConversionMap> = input.iter()
        .skip(1)
        .map(|descs| descs.iter().skip(1).map(|desc| ConversionRange::new(&desc)).collect())
        .map(|ranges| ConversionMap { ranges })
        .collect();

    let smallest_location: u64 = seeds.into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, map| map.convert(acc)))
        .min()
        .unwrap();
    
    dbg!(smallest_location);
    
    Ok(())
}

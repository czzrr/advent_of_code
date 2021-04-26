use crate::utils; 
use std::collections::HashSet;

pub fn a() {
    let lines: Vec<String> = utils::read_lines("src/day6.txt");

    let groups: Vec<&[String]> = lines
        .split(|line| line == "")
        .collect();

    let sum: u32 = groups
        .iter()
        .map(|group| group
            .join("")
            .chars()
            .collect::<HashSet<_>>()
            .len() as u32)
        .sum();

    println!("Sum is {}", sum);
}
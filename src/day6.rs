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

pub fn b() {
    let lines: Vec<String> = utils::read_lines("src/day6.txt");

    let groups: Vec<&[String]> = lines
        .split(|line| line == "")
        .collect();

    let shared_answers : Vec<HashSet<char>> = groups
        .iter()
        .map(|group| {
            let mut hash_sets_iter = group
                .iter()
                .map(|answer| answer
                .chars()
                .collect::<HashSet<char>>());

            // Compute intersection of the group's answers
            let mut intersection = hash_sets_iter.next().unwrap().clone();
            for other in hash_sets_iter {
                intersection.retain(|e| other.contains(e));
            }
            intersection
        })
        .collect();

    let sum: u32 = shared_answers
        .iter()
        .map(|hs| hs.len() as u32)
        .sum();

    println!("Sum is {:?}", sum);
}
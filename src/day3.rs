use crate::utils;

pub fn a() {
    let lines: Vec<String> = utils::read_lines("src/day3.txt");
    let mut treelines : Vec<Vec<char>> = Vec::new();
    for line in &lines {
        treelines.push(line.chars().collect());
    }
    
    let trees_hit = ride(&treelines, (3, 1));
    println!("Trees hit: {}", trees_hit);
}

pub fn b() {
    let lines: Vec<String> = utils::read_lines("src/day3.txt");
    let mut treelines : Vec<Vec<char>> = Vec::new();
    for line in &lines {
        treelines.push(line.chars().collect());
    }
    
    let slopes: Vec<(u8, u8)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product = slopes
        .iter()
        .map(|slope| ride(&treelines, *slope))
        .fold(1, |acc, x| acc * x);
    println!("Product of trees hit: {}", product);
}

fn ride(treelines: &Vec<Vec<char>>, slope: (u8, u8)) -> u32 {
    let rows = treelines.len();
    let columns = treelines[0].len();
    let mut row = 0;
    let mut column = 0;
    let mut trees_hit = 0;
    while row < rows {
        if treelines[row][column] == '#' {
            trees_hit += 1;
        }
        row += usize::from(slope.1);
        column = (column + usize::from(slope.0)) % columns;
    }
    trees_hit
}

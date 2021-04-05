use std::fs;

pub fn read_lines(filename: &str) -> Vec<String> {
    let contents = match fs::read_to_string(filename) { // Returns a Result<String>
        Ok(cnts) => cnts, // Get String from Result<String>
        Err(error) => panic!("Problem opening file: {}", error), // Quit at error
    };
    let contents : Vec<String> = contents
        .lines() // Separate contents into lines, returning an iterator
        .map(|s| s.to_string()) // Map string slices to String's
        .collect(); // From iterator to collection
    contents
}

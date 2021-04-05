use super::utils;

pub fn a() {
    let contents : Vec<String> = utils::read_lines("day1.txt");
    let entries : Vec<i32> = contents
        .iter() // Get iterator so we can map
        .map(|s| match s.parse() { // Parse each string into an integer
            Ok(n) => n,
            Err(error) => panic!("Error when parsing: {}", error),
        })
        .collect();

    for x in &entries {
        for y in &entries {
            if x + y == 2020 {
                println!("{}", x * y);
                return;
            }
        }
    }
}

pub fn b() {
    let contents : Vec<String> = utils::read_lines("day1.txt");
    let entries : Vec<i32> = contents
        .iter() // Get iterator so we can map
        .map(|s| match s.parse() { // Parse each string into an integer
            Ok(n) => n,
            Err(error) => panic!("Error when parsing: {}", error),
        })
        .collect();

    for x in &entries {
        for y in &entries {
	    for z in &entries {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return;
                }
	    }
        }
    }
}

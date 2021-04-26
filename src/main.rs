use std::env;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    if len != 4 {
        panic!("Expected 3 arguments, got {}", len - 1);
    }
    let year = args[1].to_owned();
    let day = &args[2];
    let part = &args[3];
    let combined = year + " " + day + " " + part;
    match combined.as_str() {
        "2020 1 a" => day1::a(),
        "2020 1 b" => day1::b(),
	    "2020 2 a" => day2::a(),
        "2020 2 b" => day2::b(),
        "2020 3 a" => day3::a(),
        "2020 3 b" => day3::b(),
        "2020 4 a" => day4::a(),
        "2020 5 a" => day5::a(),
        "2020 5 b" => day5::b(),
        "2020 6 a" => day6::a(),
        _ => println!("Parse error"),
    }
}



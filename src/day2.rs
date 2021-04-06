use super::utils;
use std::convert::TryFrom;

extern crate nom;

use nom::{
    IResult,
    character::complete::{char, anychar},
    bytes::complete::{take_while},
    combinator::map_res,
};

#[derive(Debug)]
struct PasswordPolicy {
    lower: u8,
    upper: u8,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid_a(&self) -> bool {
        let count = self.password
            .chars()
            .filter(|c| c == &self.letter)
            .count();
        let count = match u8::try_from(count) {
            Ok(n) => n,
            Err(error) => panic!("Error when converting usize to u8: {}", error),
        };
        self.lower <= count && count <= self.upper
    }

    fn is_valid_b(&self) -> bool {
        let psw: &Vec<char> = &self.password.chars().collect();
        let lower = usize::from(self.lower) - 1;
        let upper = usize::from(self.upper) - 1;
        (psw[lower] == self.letter && psw[upper] != self.letter)
            || (psw[lower] != self.letter && psw[upper] == self.letter)
    }
}

pub fn a() {
    let lines: Vec<String> = utils::read_lines("day2.txt");
    
    let num_valid_password_policies = lines
        .iter() // Get iterator so we can map
        .map(|s| match parse_password_policy(s) { // Parse each line into a PasswordPolicy
            Ok((_, pp)) => pp,
            Err(error) => panic!("Error when parsing: {}", error),
        })
        .filter(|pp| pp.is_valid_a())
        .count();

    println!("{}", num_valid_password_policies);
}

pub fn b() {
    let lines: Vec<String> = utils::read_lines("day2.txt");
    
    let num_valid_password_policies = lines
        .iter() // Get iterator so we can map
        .map(|s| match parse_password_policy(s) { // Parse each line into a PasswordPolicy
            Ok((_, pp)) => pp,
            Err(error) => panic!("Error when parsing: {}", error),
        })
        .filter(|pp| pp.is_valid_b())
        .count();

    println!("{}", num_valid_password_policies);
}

fn parse_password_policy(input: &str) -> IResult<&str, PasswordPolicy> {
    let (input, lower) = digit(input)?;
    let (input, _) = char('-')(input)?;
    let (input, upper) = digit(input)?;
    let (input, _) = char(' ')(input)?; // whitespace
    let (input, letter) = anychar(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = char(' ')(input)?; // whitespace
    let (input, password) = take_while(char::is_alphabetic)(input)?;
    Ok((input, PasswordPolicy {lower, upper, letter, password: password.to_string(),}))
}

fn digit(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while(is_dec_digit),
        from_dec
    )(input)
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn from_dec(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse()
}

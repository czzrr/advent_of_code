use crate::utils;

extern crate nom;

use nom::{
    IResult,
    character::complete::char,
    bytes::complete::take_while,
    multi::many0,
};

#[derive(Debug, PartialEq)]
struct Passport {
    fvps: Vec<FieldValuePair>
}

impl Passport {
    fn is_valid(&self) -> bool {
        (self.fvps.len() == 7 &&
         !self.fvps
         .iter()
         .any(|fvp| fvp.field == "cid")) ||
         self.fvps.len() == 8
     }
}

#[derive(Debug, PartialEq)]
struct FieldValuePair {
    field: String,
    value: String
}

pub fn a() {
    let lines: Vec<String> = utils::read_lines("src/day4.txt");
    
    let xs: Vec<String> = lines
        .iter()
        .fold((Vec::new(), Vec::new()),
              |mut acc, s|
              if s.len() != 0 { // If line is not blank
                  acc.1.push(s.clone());
                  acc
              } else {
                  acc.0.push(acc.1);
                  acc.1 = Vec::new();
                  acc
              })
        .0
        .iter()
        .map(|v| v.join(" "))
        .collect();

    let passports : Vec<Passport> = xs
        .iter()
        .map(|s| match parse_passport(s) {
            Ok((_, pp)) => pp,
            Err(error) => panic!("Error when parsing: {}", error), 
        })
        .collect();
    
    let num_valid_passports = passports
        .iter()
        .filter(|pp| pp.is_valid())
        .count();
    println!("Number of \"valid\" passports: {}", num_valid_passports);
}

fn parse_tag(input: &str) -> IResult<&str, FieldValuePair> {
    let (input, field) = take_while(|c| c != ':')(input)?;
    let (input, _) = char(':')(input)?;
    let (input, value) = take_while(|c| c != ' ')(input)?;
    let (input, _) = take_while(|c| c == ' ')(input)?;

    Ok((input, FieldValuePair {
        field: field.to_string(),
        value: value.to_string()
    }))
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (input, fvps) = many0(parse_tag)(input)?;
        Ok((input, Passport {fvps}))
}


// -- Parsing --



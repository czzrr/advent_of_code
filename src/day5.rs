use crate::utils; 
use std::convert::TryInto;

#[derive(Debug)]
struct EncodedSeat {
    row_direction: [u8; 7],
    column_direction: [u8; 3]
}

pub fn a() {
    let lines: Vec<String> = utils::read_lines("src/day5.txt");
    
    let encoded_seats: Vec<EncodedSeat> = lines
        .iter()
        .map(|line| parse_encoded_seat(line).expect("Error when parsing directions"))
        .collect();

    for encoded_seat in encoded_seats {
        println!("{:?}", encoded_seat);
    }

}

fn parse_encoded_seat(input: &str) -> Option<EncodedSeat> {
    if input.len() != 10 {
        return None
    }
    let row_direction: [u8; 7] = input[0..7].as_bytes().try_into().ok()?;
    let column_direction: [u8; 3] = input[7..10].as_bytes().try_into().ok()?;

    Some(EncodedSeat{row_direction, column_direction})
}
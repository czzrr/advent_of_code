use crate::utils; 
use std::convert::TryInto;

#[derive(Debug)]
struct EncodedSeat {
    row_direction: [u8; 7],
    column_direction: [u8; 3]
}

impl EncodedSeat {
    fn new(input: &str) -> Option<EncodedSeat> {
        if input.len() != 10
            || !input[0..7].chars().all(|c| c == 'F' || c == 'B')
            || !input[7..10].chars().all(|c| c == 'L' || c == 'R')
        {
            return None
        };

        let input = input.as_bytes();
        let row_direction: [u8; 7] = input[0..7].try_into().ok()?;
        let column_direction: [u8; 3] = input[7..10].try_into().ok()?;
    
        Some(EncodedSeat{ row_direction, column_direction })
    }
}

#[derive(Debug)]
struct SeatPosition {
    row: u8,
    column: u8
}

impl SeatPosition {
    fn new(encoded_seat: EncodedSeat) -> SeatPosition {
        let row = encoded_seat.row_direction
            .iter()
            .fold((0, 127), |(l, r), c| {
                match *c as char {
                    'F' => (l, (l + r) / 2),
                    _ => ((l + r) / 2, r),
                }
            }).1;
            
        let column = encoded_seat.column_direction
        .iter()
        .fold((0, 7), |(l, r), c| {
            match *c as char {
                'L' => (l, (l + r) / 2),
                _ => ((l + r) / 2, r),
            }
        }).1;

        SeatPosition { row, column }
    }
}

pub fn a() {
    let lines: Vec<String> = utils::read_lines("src/day5.txt");
    
    let highest_seat_number = lines
        .iter()
        .map(|line| {
            let encoded_seat = EncodedSeat::new(line).expect("Error when parsing encoded seat");
            let seat_position = SeatPosition::new(encoded_seat);
            let row = seat_position.row as u32;
            let column = seat_position.column as u32;
            8 * row + column
        })
        .max()
        .unwrap();
    println!("The highest seat number is {}", highest_seat_number);

}

pub fn b() {
    let lines: Vec<String> = utils::read_lines("src/day5.txt");
    
    let mut seat_ids: Vec<u32> = lines
        .iter()
        .map(|line| {
            let encoded_seat = EncodedSeat::new(line).expect("Error when parsing encoded seat");
            let seat_position = SeatPosition::new(encoded_seat);
            let row = seat_position.row as u32;
            let column = seat_position.column as u32;
            8 * row + column
        })
        .collect();
    seat_ids.sort();

    let mut my_seat_id = None;

    for (i, seat_id) in seat_ids[1..].iter().enumerate() {
        if seat_ids[i] + 2 == *seat_id {
            my_seat_id = Some(seat_id - 1);
        }
    }

    match my_seat_id {
        Some(id) => println!("My seat id is {}", id),
        None => println!("My seat id was not found :("),
    };
}
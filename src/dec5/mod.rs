use std::io::{self, BufRead, Read};

#[derive(Debug, Eq, PartialEq, Clone)]
struct SeatId(usize);

impl From<String> for SeatId {
    fn from(str: String) -> Self {
        let chars = str.chars();
        let mut row_low = 0;
        let mut row_high = 127;

        let mut column_low = 0;
        let mut column_high = 7;

        for char in chars {
            if char == 'F' {
                row_high = (row_low + row_high) / 2;
            } else if char == 'B' {
                row_low = (row_low + row_high) / 2 + 1;
            } else if char == 'R' {
                column_low = (column_high + column_low) / 2 + 1;
            } else if char == 'L' {
                column_high = (column_low + column_high) / 2;
            }
        }

        SeatId(row_low * 8 + column_low)
    }
}

fn read_seat_ids<R: Read>(reader: R) -> Vec<SeatId> {
    let mut ids = Vec::new();
    for line in io::BufReader::new(reader).lines() {
        ids.push(line.unwrap())
    }
    ids.iter()
        .map(|id_str| SeatId::from(id_str.to_string()))
        .collect()
}

fn max_seat(seat_ids: Vec<SeatId>) -> usize {
    seat_ids.iter().map(|val| val.0).max().unwrap()
}

fn min_seat(seat_ids: Vec<SeatId>) -> usize {
    seat_ids.iter().map(|val| val.0).min().unwrap()
}

fn find_empty_seat(seat_ids: Vec<SeatId>) -> usize {
    let min = min_seat(seat_ids.clone());
    let max = max_seat(seat_ids.clone());

    let all_seats: Vec<usize> = (min..max).collect();
    let empty = all_seats
        .into_iter()
        .filter(|val| !seat_ids.contains(&SeatId(*val)))
        .collect::<Vec<usize>>();
    return empty[0];
}

pub fn solve_puzzle<R: Read>(reader: R) {
    let seat_ids: Vec<SeatId> = read_seat_ids(reader);
    let max_seat = max_seat(seat_ids.clone());
    let empty_seat = find_empty_seat(seat_ids);

    println!("max seat id = {:#?}", max_seat);
    println!("empty seat = {}", empty_seat);
}

#[cfg(test)]
mod tests {
    use crate::dec5::{max_seat, read_seat_ids, SeatId};
    use std::fs::File;

    #[test]
    fn test_max() {
        let f = File::open("src/dec5/input.txt").unwrap();
        let seat_ids = read_seat_ids(f);
        assert_eq!(max_seat(seat_ids.clone()), 858);
    }

    #[test]
    fn test_simple() {
        let input = String::from("FBFBBFFRLR");
        let seat = SeatId::from(input);
        assert_eq!(seat, SeatId(357));
    }
}

use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

fn solve_puzzle(path: &Path) {
    let input = read_to_string(path).unwrap();
    let (ranges, flatten_tickets) = parse_input_part_1(input);
    println!(
        "sum of not in any range: {}",
        sum_of_not_in_range(ranges, flatten_tickets)
    );
}

fn parse_input_part_1(input: String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let ranges_regex = Regex::new("([\\d]+-[\\d]+)").unwrap();
    let range_elem_regex = Regex::new("([\\d]+)-([\\d]+)").unwrap();

    let mut ranges = Vec::new();

    for range in ranges_regex.captures_iter(&input) {
        let range = range[0].trim();
        let range_elems = range_elem_regex.captures(range).unwrap();

        ranges.push((
            range_elems[1].to_string().parse::<u64>().unwrap(),
            range_elems[2].to_string().parse::<u64>().unwrap(),
        ));
    }

    let mut flatten_tickets = Vec::new();
    let flatten_tickets_regex = Regex::new("([0-9]+)").unwrap();
    for line in input.lines().skip(
        input
            .lines()
            .find_position(|elem| elem.contains("nearby tickets:"))
            .unwrap()
            .0
            + 1,
    ) {
        for capture in flatten_tickets_regex.captures_iter(line) {
            flatten_tickets.push(capture[1].parse::<u64>().unwrap())
        }
    }

    (ranges, flatten_tickets)
}

fn val_in_ranges(value: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        if value >= range.0 && value <= range.1 {
            return true;
        }
    }
    return false;
}

fn sum_of_not_in_range(ranges: Vec<(u64, u64)>, flatten_tickets: Vec<u64>) -> u64 {
    let mut res = 0;
    for ticket in flatten_tickets {
        if !val_in_ranges(ticket, &ranges) {
            res += ticket;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::dec16::{parse_input_part_1, solve_puzzle, sum_of_not_in_range};
    use std::path::Path;

    #[test]
    fn test_part_1() {
        let input = r#"
            class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12
        "#
        .to_string();
        let expected_ranges = vec![(1u64, 3), (5, 7), (6, 11), (33, 44), (13, 40), (45, 50)];
        let expected_tickets = vec![7u64, 3, 47, 40, 4, 50, 55, 2, 20, 38, 6, 12];
        let (ranges, tickets) = parse_input_part_1(input);
        assert_eq!(ranges, expected_ranges);
        assert_eq!(tickets, expected_tickets);

        assert_eq!(sum_of_not_in_range(ranges, tickets), 71);
    }

    #[test]
    fn call_solve_puzzle() {
        solve_puzzle(Path::new("src/dec16/input.txt"));
    }
}

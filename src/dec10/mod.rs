use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_input(path: &Path) -> Vec<u64> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut res = Vec::new();
    for line in lines {
        res.push(line.unwrap().parse::<u64>().unwrap());
    }

    res
}

fn updated_input(input: Vec<u64>) -> Vec<u64> {
    let mut sorted = input.clone();
    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);

    sorted
}

fn diff_of_1_mul_diff_of_3(input: Vec<u64>) -> u64 {
    let mut nr_of_1_diff = 0;
    let mut nr_of_3_diff = 0;
    let sorted = updated_input(input);
    for i in 1..sorted.len() {
        let diff = sorted[i] - sorted[i - 1];
        if diff == 1 {
            nr_of_1_diff += 1;
        }
        if diff == 3 {
            nr_of_3_diff += 1;
        }
    }
    nr_of_1_diff * nr_of_3_diff
}

fn all_valid_combinations(input: Vec<u64>) -> u64 {
    let mut groups: Vec<u64> = Vec::new();
    let sorted = updated_input(input);
    let mut local_count: u64 = 0;
    for i in 1..sorted.len() {
        let diff = sorted[i] - sorted[i - 1];
        if diff == 1 {
            local_count += 1;
        } else if diff == 2 {
            panic!("not working");
        } else {
            groups.push(local_count);
            local_count = 0;
        }
    }
    // The beautiful values below that represent the possible combinations
    // were computed with a recursive solution that was too slow for the large input.
    // Said function: `all_valid_combinations_recursion`.
    groups
        .into_iter()
        .filter(|elem| *elem >= 2u64)
        .map(|elem| match elem {
            2 => 2,
            3 => 4,
            4 => 7,
            5 => 13,
            _ => panic!("too many consecutive values."),
        })
        .fold(1u64, |acc, elem| acc * elem)
}

fn valid_from(input: &Vec<u64>, correct_solutions: &mut Vec<Vec<u64>>) {
    for skip in 1..(input.len() - 1) {
        if input[skip + 1] - input[skip - 1] <= 3 {
            let try_from = (*input)
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != skip)
                .map(|(_, v)| *v)
                .collect_vec();
            if !correct_solutions.contains(&try_from) {
                correct_solutions.push(try_from.clone());
                valid_from(&try_from, correct_solutions);
            }
        }
    }
}

fn all_valid_combinations_recursion(input: Vec<u64>) -> u64 {
    let input = updated_input(input.clone());
    let mut correct_solutions = Vec::<Vec<u64>>::new();
    valid_from(&input, &mut correct_solutions);
    (correct_solutions.len() + 1) as u64
}

pub fn solve_puzzle(path: &Path) {
    let input = read_input(path);
    println!(
        "Nr of 1-jolt diff multiplied by the nr of 3-jol = {}",
        diff_of_1_mul_diff_of_3(input.clone())
    );

    println!(
        "Nr of all possible combinations: {}",
        all_valid_combinations(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(diff_of_1_mul_diff_of_3(input.clone()), 35);
        assert_eq!(all_valid_combinations_recursion(input), 8);

        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(diff_of_1_mul_diff_of_3(input.clone()), 220);
        assert_eq!(all_valid_combinations(input), 19208);
    }

    #[test]
    fn test_print_valid_comb() {
        let vec = vec![1, 2];
        println!(
            "# valid combinations for len={} is {}",
            vec.len() + 1,
            all_valid_combinations_recursion(vec.clone())
        );

        let vec = vec![1, 2, 3];
        println!(
            "# valid combinations for len={} is {}",
            vec.len() + 1,
            all_valid_combinations_recursion(vec.clone())
        );

        let vec = vec![1, 2, 3, 4];
        println!(
            "# valid combinations for len={} is {}",
            vec.len() + 1,
            all_valid_combinations_recursion(vec.clone())
        );

        let vec = vec![1, 2, 3, 4, 5];
        println!(
            "# valid combinations for len={} is {}",
            vec.len() + 1,
            all_valid_combinations_recursion(vec.clone())
        );
    }

    #[test]
    fn call_solve_puzzle() {
        solve_puzzle(Path::new("src/dec10/input.txt"));
    }
}

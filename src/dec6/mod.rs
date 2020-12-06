use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

fn read_all_group_qs(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .split_terminator("\n\n")
        .map(|val| val.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn count_all_yes(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|str| {
            str.replace('\n', "")
                .replace(' ', "")
                .chars()
                .unique()
                .collect()
        })
        .collect::<Vec<String>>()
        .iter()
        .fold(0, |acc, str| acc + str.len())
}

pub fn count_everyone_yes(input: Vec<String>) -> usize {
    let mut count = 0;
    for str in input {
        let groups = str
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|person| person.replace('\n', "").replace(' ', "").chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();

        let first = groups[0].clone();
        let intersection = groups
            .iter()
            .fold(first, |acc, elem| acc.intersection(elem).cloned().collect());
        count += intersection.len();
    }

    count
}

pub fn solve_puzzle(reader: &Path) {
    let answers = read_all_group_qs(reader);
    println!(
        "nr of unique yes answers: {}",
        count_all_yes(answers.clone())
    );
    println!(
        "nr of all groups with same questions answered yes = {}",
        count_everyone_yes(answers)
    );
}

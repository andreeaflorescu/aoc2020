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

fn has_sum(input: Vec<u64>, sum: u64) -> bool {
    input
        .iter()
        .combinations(2)
        .any(|res| res[0] + res[1] == sum)
}

fn range_with_sum(input: Vec<u64>, sum: u64) -> (u64, u64) {
    for i in 0..input.len() - 1 {
        let mut local_min = u64::MAX;
        let mut local_max = u64::MIN;
        let res = input
            .iter()
            .skip(i)
            .try_fold(0, |mut acc, elem| {
                if acc < sum {
                    if *elem > local_max {
                        local_max = *elem;
                    }
                    if *elem < local_min {
                        local_min = *elem;
                    }
                    acc += elem;
                    Ok(acc)
                } else {
                    if acc == sum {
                        // return an error so that we can stop the fold.
                        Err("found")
                    } else {
                        Err("not found")
                    }
                }
            })
            .unwrap_err();

        match res {
            "not found" => {}
            "found" => {
                return (local_min, local_max);
            }
            _ => panic!("shouldn't happen"),
        }
    }

    (0, 0)
}

fn find_first_wrong_number(input: Vec<u64>, preamble: usize) -> u64 {
    *input
        .iter()
        .into_iter()
        .enumerate()
        .find(|(index, elem)| {
            if *index < preamble {
                false
            } else {
                !has_sum(input.to_vec()[index - preamble..*index].to_vec(), **elem)
            }
        })
        .unwrap()
        .1
}

pub fn solve_puzzle(path: &Path) {
    let input = read_input(path);
    let nr = find_first_wrong_number(input.clone(), 25);
    println!("first wrong number: {}", nr);

    let range_with_sum = range_with_sum(input, nr);
    println!(
        "min + max of range: {}",
        range_with_sum.0 + range_with_sum.1
    );
}

#[test]
fn test_simple() {
    let input = vec![
        35u64, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(find_first_wrong_number(input.clone(), 5), 127);
    assert_eq!(range_with_sum(input, 127), (15, 47));
}

#[test]
fn call_solve_puzzle() {
    solve_puzzle(Path::new("src/dec9/input.txt"));
}

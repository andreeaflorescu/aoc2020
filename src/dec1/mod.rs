use std::io;
use std::io::{BufRead, Read};

fn into_vec<T: Read>(file: T) -> Vec<u32> {
    let reader = io::BufReader::new(file);
    let mut result = Vec::<u32>::new();
    for line in reader.lines() {
        line.map(|str| result.push(str.parse::<u32>().unwrap()))
            .unwrap();
    }
    result
}

fn sum_of_x(input: Vec<u32>, nr: usize) -> Vec<Vec<u32>> {
    let mut combinations = Vec::<Vec<u32>>::new();

    for i in 0..input.len() - nr {
        let mut vec = Vec::new();
        let mut iter = input[i..].iter().step_by(nr);
        while let Some(x) = iter.next() {
            vec.push(*x);
        }

        combinations.push(vec);
    }
    combinations
}

fn sum_of_3_2020(vec: Vec<u32>) -> (u32, u32, u32) {
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            for k in 0..vec.len() {
                if i != j && j != k && i != k && vec[i] + vec[j] + vec[k] == 2020 {
                    return (vec[i], vec[j], vec[k]);
                }
            }
        }
    }
    (0, 0, 0)
}

fn sum_of_2_2020(vec: Vec<u32>) -> (u32, u32) {
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if i != j && vec[i] + vec[j] == 2020 {
                return (vec[i], vec[j]);
            }
        }
    }
    (0, 0)
}

pub fn solve_puzzle<T: Read>(reader: T) {
    let input = into_vec(reader);
    let (x, y) = sum_of_2_2020(input.clone());
    println!("product of 2={}", x * y);
    let (x, y, z) = sum_of_3_2020(input);
    println!("product of 3={}", x * y * z);
}

#[cfg(test)]
mod tests {
    use crate::dec1::sum_of_x;

    #[test]
    fn test_combinations() {
        let vec = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let combinations = sum_of_x(vec, 3);
        println!("{:#?}", combinations);
    }
}

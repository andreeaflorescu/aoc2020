mod dec1;
mod dec2;
mod dec3;

use dec1::solve_puzzle as solve_puzzle_1;
use dec2::solve_puzzle as solve_puzzle_2;
use dec3::solve_puzzle as solve_puzzle_3;
use std::fs::File;

fn main() {
    println!("Day 1 puzzle:");
    solve_puzzle_1(File::open("src/dec1/input.txt").unwrap());
    println!("=========================\n");

    println!("Day 2 puzzle:");
    solve_puzzle_2(File::open("src/dec2/input.txt").unwrap());
    println!("=========================\n");

    println!("Day 3 puzzle:");
    solve_puzzle_3(File::open("src/dec3/input.txt").unwrap());
    println!("=========================\n");
}

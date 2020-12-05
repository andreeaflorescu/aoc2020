mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;

use dec1::solve_puzzle as solve_puzzle_1;
use dec2::solve_puzzle as solve_puzzle_2;
use dec3::solve_puzzle as solve_puzzle_3;
use dec4::solve_puzzle as solve_puzzle_4;
use dec5::solve_puzzle as solve_puzzle_5;
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

    println!("Day 4 puzzle:");
    solve_puzzle_4(File::open("src/dec4/input.txt").unwrap());
    println!("=========================\n");

    println!("Day 5 puzzle:");
    solve_puzzle_5(File::open("src/dec5/input.txt").unwrap());
    println!("=========================\n");
}

mod dec1;
mod dec10;
mod dec2;
mod dec3;
mod dec4;
mod dec5;
mod dec6;
mod dec7;
mod dec8;
mod dec9;

use dec1::solve_puzzle as solve_puzzle_1;
use dec2::solve_puzzle as solve_puzzle_2;
use dec3::solve_puzzle as solve_puzzle_3;
use dec4::solve_puzzle as solve_puzzle_4;
use dec5::solve_puzzle as solve_puzzle_5;
use dec6::solve_puzzle as solve_puzzle_6;
use dec7::solve_puzzle as solve_puzzle_7;
use dec8::solve_puzzle as solve_puzzle_8;
use dec9::solve_puzzle as solve_puzzle_9;

use std::fs::File;
use std::path::Path;

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

    println!("Day 6 puzzle:");
    solve_puzzle_6(Path::new("src/dec6/input.txt"));
    println!("=========================\n");

    println!("Day 7 puzzle:");
    solve_puzzle_7(Path::new("src/dec7/input.txt"));
    println!("=========================\n");

    println!("Day 8 puzzle:");
    solve_puzzle_8(Path::new("src/dec8/input.txt"));
    println!("=========================\n");

    println!("Day 9 puzzle:");
    solve_puzzle_9(Path::new("src/dec9/input.txt"));
    println!("=========================\n");
}

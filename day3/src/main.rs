use std::{process};

use day3::{solution1, solution2};

fn main() {
    if let Err(e) = solution1(include_str!("../input.txt")) {
        println!("Application error: {e}");
        process::exit(1);
    }

    let solution1 = solution1(include_str!("../input.txt")).unwrap();
    println!("The solution to part1 is: {solution1}");

    let solution1 = solution2(include_str!("../input.txt")).unwrap();
    println!("The solution to part2 is: {solution1}");
}

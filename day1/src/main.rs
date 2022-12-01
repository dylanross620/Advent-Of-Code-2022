use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("------ Puzzle 1 -----\n");
    puzzle_1();

    println!("\n------ Puzzle 2 -----\n");
    puzzle_2();
}

fn puzzle_1() {
    let mut largest: u64 = 0;
    let mut cur: u64 = 0;
    if let Ok(lines) = read_lines("puzzle_input") {
        for line in lines {
            if let Ok(calories) = line {
                let cal = calories.trim();
                if cal.len() == 0 {
                    if cur > largest {
                        largest = cur;
                    }
                    cur = 0;
                }
                else if let Ok(calories_int) = calories.parse::<u64>() {
                    cur += calories_int;
                }
                else {
                    println!("Error on {}", calories)
                }
            }
        }
    }

    println!("Largest is {}", largest);

}

fn puzzle_2() {
    let mut sums: Vec<u64> = Vec::new();
    let mut cur: u64 = 0;
    if let Ok(lines) = read_lines("puzzle_input") {
        for line in lines {
            if let Ok(calories) = line {
                let cal = calories.trim();
                if cal.len() == 0 {
                    sums.push(cur);
                    cur = 0;
                }
                else if let Ok(calories_int) = calories.parse::<u64>() {
                    cur += calories_int;
                }
                else {
                    println!("Error on {}", calories)
                }
            }
        }
    }

    sums.sort();
    sums.reverse();
    let total: u64 = sums.iter().take(3).sum();

    println!("Total is {}", total);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

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

fn get_points(opp: &Moves, play: &Moves) -> usize {
    if *play == Moves::Rock {
        (Moves::Rock as usize) + match opp {
            Moves::Rock => 3,
            Moves::Paper => 0,
            Moves::Scissors => 6,
        }
    }
    else if *play == Moves::Scissors {
        (Moves::Scissors as usize) + match opp {
            Moves::Rock => 0,
            Moves::Paper => 6,
            Moves::Scissors => 3,
        }
    }
    else {
        (Moves::Paper as usize) + match opp {
            Moves::Rock => 6,
            Moves::Paper => 3,
            Moves::Scissors => 0,
        }
    }
}

fn puzzle_1() {
    let mut move_map = HashMap::new();
    move_map.insert("A", Moves::Rock);
    move_map.insert("B", Moves::Paper);
    move_map.insert("C", Moves::Scissors);
    move_map.insert("X", Moves::Rock);
    move_map.insert("Y", Moves::Paper);
    move_map.insert("Z", Moves::Scissors);

    let mut sum = 0;

    if let Ok(lines) = read_lines("puzzle_input") {
        for line in lines {
            if let Ok(instruction) = line {
                let inst: Vec<&str> = instruction.split(' ').collect();
                sum += get_points(move_map.get(inst[0]).unwrap(), move_map.get(inst[1]).unwrap());
            }
        }
    }

    println!("Sum: {}", sum);
}

fn get_des_result(opp: &Moves, des: &str) -> Moves {
    if des == "Y" {
        return opp.clone();
    }

    if *opp == Moves::Rock {
        match des {
            "X" => Moves::Scissors,
            "Z" => Moves::Paper,
            _ => panic!("Unknown des")
        }
    }
    else if *opp == Moves::Paper {
        match des {
            "X" => Moves::Rock,
            "Z" => Moves::Scissors,
            _ => panic!("Unknown des")
        }
    }
    else {
        match des {
            "X" => Moves::Paper,
            "Z" => Moves::Rock,
            _ => panic!("Unknown des")
        }
    }
}

fn puzzle_2() {
    let mut move_map = HashMap::new();
    move_map.insert("A", Moves::Rock);
    move_map.insert("B", Moves::Paper);
    move_map.insert("C", Moves::Scissors);

    let mut sum = 0;

    if let Ok(lines) = read_lines("puzzle_input") {
        for line in lines {
            if let Ok(instruction) = line {
                let inst: Vec<&str> = instruction.split(' ').collect();
                let opp = move_map.get(inst[0]).unwrap();
                let play = get_des_result(&opp, inst[1]);
                sum += get_points(&opp, &play);
            }
        }
    }

    println!("Sum: {}", sum);
}

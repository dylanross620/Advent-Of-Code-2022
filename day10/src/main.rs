use std::fs;
use std::collections::LinkedList;

#[derive(Clone, Copy, Debug)]
struct Instruction {
    cycles: u8,
    diff: isize,
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines = lines.split_terminator("\n").map(|l| l.trim()).filter(|l| l.len() > 0);

    let mut insts: LinkedList<Instruction> = LinkedList::new();
    for line in lines {
        let l: Vec<&str> = line.split(" ").collect();

        insts.push_back(
            match l[0] {
                "noop" => Instruction { cycles: 1, diff: 0 },
                "addx" => Instruction { cycles: 2, diff: l[1].parse().unwrap() },
                _ => panic!("Unknown instruction {}", l[0])
            }
        );
    }

    println!("----- Puzzle 1 -----");
    let mut tmp = insts.clone();
    puzzle_1(&mut tmp);

    println!("----- Puzzle 2 -----");
    puzzle_2(&mut insts);
}

fn puzzle_1(insts: &mut LinkedList<Instruction>) {
    let mut val = 1;
    let mut sum = 0;

    let mut cycle_count = 0;
    let mut cur_inst = insts.pop_front().unwrap();

    loop {
        cycle_count += 1;
        let signal = val * cycle_count;
        if cycle_count == 20 {
            sum += signal;
        }
        else if cycle_count > 20 && (cycle_count - 20) % 40 == 0 {
            sum += signal;
        }

        cur_inst.cycles -= 1;

        if cur_inst.cycles == 0 {
            val += cur_inst.diff;

            if let Some(inst) = insts.pop_front() {
                cur_inst = inst;
            }
            else {
                break;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn puzzle_2(insts: &mut LinkedList<Instruction>) {
    let mut val: isize = 1;

    let mut cycle_count: isize = 0;
    let mut cur_inst = insts.pop_front().unwrap();

    loop {
        if (cycle_count % 40 - val).abs() <= 1 {
            print!("#");
        }
        else {
            print!(".");
        }

        if cycle_count % 40 == 39 {
            println!("");
        }

        cycle_count += 1;

        cur_inst.cycles -= 1;

        if cur_inst.cycles == 0 {
            val += cur_inst.diff;

            if let Some(inst) = insts.pop_front() {
                cur_inst = inst;
            }
            else {
                break;
            }
        }
    }
}

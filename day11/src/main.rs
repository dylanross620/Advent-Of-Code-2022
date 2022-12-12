use std::{collections::LinkedList, cell::RefCell};

struct Monkey {
    items: LinkedList<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    count: usize,
}

impl Monkey {
    fn take_turn(&mut self, monkey_list: &Vec<RefCell<Monkey>>, worry_decrease: bool) {
        loop {
            if let Some(item) = self.items.pop_front() {
                // Product of all remainders being taken
                let prod = 13 * 2 * 7 * 17 * 5 * 11 * 3 * 19;

                let level = ((self.op)(item) / if worry_decrease { 3 } else { 1 }) % prod;
                let target = (self.test)(level);

                monkey_list[target].borrow_mut().items.push_back(level);
                self.count += 1;
            }
            else {
                break;
            }
        }
    }
}

fn main() {
    // Manually creating a list of Monkeys to match the input, parsing is too much work
    let mut monkeys: Vec<RefCell<Monkey>> = Vec::new();
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([84, 72, 58, 51]),
        op: Box::new(|old| old * 3),
        test: Box::new(|v| if v % 13 == 0 { 1 } else { 7 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([88, 58, 58]),
        op: Box::new(|old| old + 8),
        test: Box::new(|v| if v % 2 == 0 { 7 } else { 5 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([93, 82, 71, 77, 83, 53, 71, 89]),
        op: Box::new(|old| old * old),
        test: Box::new(|v| if v % 7 == 0 { 3 } else { 4 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([81, 68, 65, 81, 73, 77, 96]),
        op: Box::new(|old| old + 2),
        test: Box::new(|v| if v % 17 == 0 { 4 } else { 6 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([75, 80, 50, 73, 88]),
        op: Box::new(|old| old + 3),
        test: Box::new(|v| if v % 5 == 0 { 6 } else { 0 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([59, 72, 99, 87, 91, 81]),
        op: Box::new(|old| old * 17),
        test: Box::new(|v| if v % 11 == 0 { 2 } else { 3 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([86, 69]),
        op: Box::new(|old| old + 6),
        test: Box::new(|v| if v % 3 == 0 { 1 } else { 0 }),
        count: 0
    }));
    monkeys.push(RefCell::new(Monkey {
        items: LinkedList::from([91]),
        op: Box::new(|old| old + 1),
        test: Box::new(|v| if v % 19 == 0 { 2 } else { 5 }),
        count: 0
    }));

    //println!("----- Puzzle 1 -----");
    //puzzle_1(&monkeys);

    println!("----- Puzzle 2 -----");
    puzzle_2(&monkeys);
}

fn puzzle_1(monkeys: &Vec<RefCell<Monkey>>) {
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            monkey.borrow_mut().take_turn(monkeys, true);
        }
    }

    let mut counts: Vec<usize> = Vec::new();
    for monkey in monkeys {
        counts.push(monkey.borrow().count);
    }

    counts.sort();
    counts.reverse();

    println!("Monkey business: {}", counts[0] * counts[1]);
}

fn puzzle_2(monkeys: &Vec<RefCell<Monkey>>) {

    for _ in 0..10000 {
        for monkey in monkeys.iter() {
            monkey.borrow_mut().take_turn(monkeys, false);
        }
    }

    let mut counts: Vec<usize> = Vec::new();
    for monkey in monkeys {
        counts.push(monkey.borrow().count);
    }

    counts.sort();
    counts.reverse();

    println!("Monkey business: {}", counts[0] * counts[1]);
}

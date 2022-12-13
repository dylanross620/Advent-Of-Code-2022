use std::{fs, cmp::Ordering};

#[derive(Eq, Clone)]
struct List {
    elems: Box<Vec<ListItem>>
}

#[derive(PartialEq, Eq, Clone)]
enum ListItem {
    List(List),
    Num(u8),
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ListItem::List(list) => {
                match other {
                    ListItem::List(other_list) => list.cmp(&other_list),
                    ListItem::Num(other_num) => {
                        let tmp = List { elems: Box::new(vec![ListItem::Num(*other_num)]) };
                        list.cmp(&tmp)
                    },
                }
            },
            ListItem::Num(num) => {
                match other {
                    ListItem::List(other_list) => {
                        let tmp = List { elems: Box::new(vec![ListItem::Num(*num)]) };
                        tmp.cmp(&other_list)
                    },
                    ListItem::Num(other_num) => num.cmp(&other_num),
                }
            },
        }
    }
}

impl PartialOrd<Self> for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.elems.len() {
            if i >= other.elems.len() {
                return Ordering::Greater;
            }

            match self.elems[i].cmp(&other.elems[i]) {
                Ordering::Equal => {},
                val => { return val; },
            };
        }

        if self.elems.len() == other.elems.len() {
            return Ordering::Equal;
        }

        Ordering::Less
    }
}

impl PartialOrd<Self> for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for List {
    fn eq(&self, other: &Self) -> bool {
        self.elems.eq(&other.elems)
    }
}

impl List {
    fn parse(input: &str) -> Self {
        let mut elems: Box<Vec<ListItem>> = Box::new(Vec::new());
        let input = &input[1..input.len() - 1];

        let mut ptr = &input[..];
        while ptr.len() > 0 {
            let comma_idx = ptr.find(",").unwrap_or(usize::MAX);
            let list_idx = ptr.find("[").unwrap_or(usize::MAX);

            if comma_idx == usize::MAX && list_idx == usize::MAX {
                elems.push(ListItem::Num(ptr.parse().unwrap()));
                ptr = &ptr[ptr.len()..];
            }
            else if comma_idx < list_idx {
                let item = &ptr[..comma_idx];
                if let Ok(n) = item.parse() {
                    elems.push(ListItem::Num(n));
                }
                ptr = &ptr[comma_idx + 1..];
            }
            else {
                let mut list_end = 1;
                let mut bracket_count = 0;
                for (i, c) in ptr.chars().enumerate() {
                    if c == '[' {
                        bracket_count += 1;
                    }
                    else if c == ']' {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            list_end = i;
                            break;
                        }
                    }
                }
                let item = &ptr[..list_end + 1];
                elems.push(ListItem::List(List::parse(item)));
                ptr = &ptr[list_end + 1..];
            }
        }

        List { elems }
    }
}

fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let lines: Vec<&str> = lines.split("\n").map(|l| l.trim()).filter(|l| l.len() > 0).collect();

    let mut packets: Vec<ListItem> = lines.iter().map(|l| ListItem::List(List::parse(l))).collect();

    println!("----- Puzzle 1 -----");
    puzzle_1(&packets);

    println!("----- Puzzle 2 -----");
    puzzle_2(&mut packets);
}

fn puzzle_1(packets: &Vec<ListItem>) {
    let mut sum = 0;

    for (i, chunk) in packets.chunks(2).enumerate() {
        if chunk[0] < chunk[1] {
            sum += i + 1;
        }
    }

    println!("Sum: {}", sum);
}

fn puzzle_2(packets: &mut Vec<ListItem>) {
    let decoder_1 = ListItem::List(List {
        elems: Box::new(vec![ListItem::List(List {
            elems: Box::new(vec![ListItem::Num(2)])
        })])
    });
    let decoder_2 = ListItem::List(List {
        elems: Box::new(vec![ListItem::List(List {
            elems: Box::new(vec![ListItem::Num(6)])
        })])
    });

    packets.push(decoder_1.clone());
    packets.push(decoder_2.clone());

    packets.sort();

    let idx_1 = packets.iter().position(|p| *p == decoder_1).unwrap() + 1;
    let idx_2 = packets.iter().position(|p| *p == decoder_2).unwrap() + 1;

    println!("Res: {}", idx_1 * idx_2)
}

use std::{fs, collections::LinkedList, cell::RefCell, rc::Rc, fmt};
use regex::Regex;

struct File {
    size: usize,
    name: String,
}

struct Dir {
    contents: Vec<Rc<RefCell<Type>>>,
    name: String,
}

enum Type {
    File(File),
    Dir(Dir),
}

impl Type {
    fn size(&self) -> usize {
        match self {
            Type::File(f) => f.size,
            Type::Dir(dir) => {
                let mut sum = 0;
                for item in &dir.contents {
                    sum += item.borrow().size();
                }
                sum
            }
        }
    }

    fn fmt_internal(&self, f: &mut fmt::Formatter, depth: u8) -> fmt::Result {
        match self {
            Type::File(file) => file.format_internal(f, depth),
            Type::Dir(dir) => dir.format_internal(f, depth)
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_internal(f, 0)
    }
}

impl Dir {
    fn format_internal(&self, f: &mut fmt::Formatter, depth: u8) -> fmt::Result {
        let res = write!(f, "{}Dir; Name: {}\n", "\t".repeat(depth.into()), self.name);

        for child in &self.contents {
            let tmp = child.borrow();
            tmp.fmt_internal(f, depth + 1);
        }

        res
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format_internal(f, 0)
    }
}

impl File {
    fn format_internal(&self, f: &mut fmt::Formatter, depth: u8) -> fmt::Result {
        write!(f, "{}File; Name: {} Size: {}\n", "\t".repeat(depth.into()), self.name, self.size)
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format_internal(f, 0)
    }
}


fn main() {
    let lines = fs::read_to_string("puzzle_input").unwrap();
    let commands = lines.split_terminator("$").filter(|g| g.len() > 0);

    let mut path: LinkedList<Rc<RefCell<Type>>> = LinkedList::new();

    for group in commands {
        let mut lines: LinkedList<&str> = group.split_terminator("\n").map(|l| l.trim()).filter(|l| l.len() > 0).collect();

        let cmd = lines.pop_front().unwrap();

        if cmd == "ls" {
            let file_regex = Regex::new(r"^([0-9]+) (.+)$").unwrap();
            let dir_regex = Regex::new(r"^dir (.*)$").unwrap();
            for line in lines {
                let mut dir = path.back().unwrap().borrow_mut();
                match &mut *dir {
                    Type::File(_) => panic!(),
                    Type::Dir(d) => {
                        for cap in file_regex.captures_iter(line) {
                            let file = File { size: cap[1].to_string().parse().unwrap(), name: cap[2].to_owned() };
                            d.contents.push(Rc::new(RefCell::new(Type::File(file))));
                        }

                        for cap in dir_regex.captures_iter(line) {
                            let tmp = Dir { contents: Vec::new(), name: cap[1].to_owned() };
                            d.contents.push(Rc::new(RefCell::new(Type::Dir(tmp))));
                        }
                    }
                };
            }
        }

        else {
            let target = cmd.split(" ").nth(1).unwrap();

            match target {
                "/" => {
                    if path.len() == 0 {
                        let root = Dir { contents: Vec::new(), name: "/".to_string() };
                        path.push_back(Rc::new(RefCell::new(Type::Dir(root))));
                    }
                    while path.len() > 1 {
                        path.pop_back();
                    }
                },
                ".." => { path.pop_back(); },
                name => {
                    let mut target: Option<Rc<RefCell<Type>>> = None;
                    {
                        let dir = path.back().unwrap().borrow_mut();
                        if let Type::Dir(d) = &*dir {
                            for subdir in &d.contents {
                                if let Type::Dir(d) = &*subdir.borrow() {
                                    if d.name == name {
                                        target = Some(Rc::clone(subdir));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    match target {
                        Some(t) => path.push_back(t),
                        None => panic!()
                    }
                }
            };
        }
    }

    println!("----- Puzzle 1 -----");
    puzzle_1(&*path.front().unwrap().borrow());

    println!("----- Puzzle 2 -----");
    puzzle_2(&*path.front().unwrap().borrow());
}

fn recursive_search(node: &Type, sum: &mut usize) {
    match node {
        Type::File(_) => return,
        Type::Dir(dir) => {
            if node.size() <= 100000 {
                *sum += node.size();
            }

            for child in &dir.contents {
                recursive_search(&*child.borrow(), sum);
            }
        }
    };
}

fn recursive_search_min(node: &Type, target: usize, cur_smallest: &mut usize) {
    match node {
        Type::File(_) => return,
        Type::Dir(dir) => {
            let size = node.size();
            if size >= target && size < *cur_smallest {
                *cur_smallest = size;
            }

            for child in &dir.contents {
                recursive_search_min(&*child.borrow(), target, cur_smallest);
            }
        }
    };
}

fn puzzle_1(root: &Type) {
    let mut sum = 0;

    recursive_search(root, &mut sum);

    println!("Sum: {}", sum);
}

fn puzzle_2(root: &Type) {
    let target = 30000000;
    let needed = target - (70000000 - root.size());

    if needed <= 0 {
        println!("No deletions necessary");
        return;
    }
    let needed = needed as usize;

    let mut smallest = usize::MAX;
    recursive_search_min(root, needed, &mut smallest);

    println!("Smallest: {}", smallest);
}

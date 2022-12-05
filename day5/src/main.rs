use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day5 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day5 FILE");
        return;
    }

    let (mut stacks, moves) = parse_input(file_path);

    for m in moves {
        stacks.move_stuff(m.0, m.1, m.2);
    }

    println!("part 1:{}", stacks.tops());

}


fn parse_input(file_path: &str) -> (Stacks, Vec<(usize, usize, usize)>) {

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let sections = text.split("\n\n").collect::<Vec<&str>>();

    let mut stacks = Stacks {
        stacks: Vec::new(),
    };

    let stack_height = sections[0].lines().count();
    let stack_count = (sections[0].lines().last().unwrap().len()+1)/4;

    for _ in 0..stack_count {
        stacks.stacks.push(Vec::new());
    }

    for line in sections[0].lines().take(stack_height-1).collect::<Vec<_>>().iter().rev() {
        let row = parse_stack(line, stack_count);
        for (i,c) in row.iter().enumerate() {
            if c != &' ' { stacks.stacks[i].push(c.clone()); }
        }
    }


    let mut moves: Vec<(usize, usize, usize)> = Vec::new();

    for line in sections[1].lines() {
        let cmove: (usize, usize, usize) = parse_move(&line);
        moves.push(cmove);
    }
    return (stacks, moves);
}

fn parse_stack(line: &str, size: usize) -> Vec<char> {

    let mut result:Vec<char> = Vec::new();
    let mut chars = line.chars();
    result.push(chars.nth(1).unwrap().clone());

    for _ in 1..size {
        result.push(chars.nth(3).unwrap().clone());
    }
    return result;
}

fn parse_move(line: &str) -> (usize, usize, usize) {

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let caps = re.captures(line).expect("Bad move input given");

    let n = caps.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
    let l = caps.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
    let r = caps.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap();

    return (n, l-1, r-1);
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

pub trait Stackable {
    fn tops(&self) -> String;
    fn top(&self, id: usize) -> char;
    fn move_stuff(&mut self, num: usize, from: usize, to: usize);
}

impl Stackable for Stacks {

    fn tops(&self) -> String {
        let mut result = String::from("");
        for (i,_) in self.stacks.iter().enumerate() {
            result.push(self.top(i));
        }
        return result;
    }

    fn top(&self, id: usize) -> char {
        return self.stacks[id].iter().last().copied().unwrap();
    }

    fn move_stuff(&mut self, num: usize, from: usize, to: usize) {
        let idx = self.stacks[from].len() - num;
        let mut moved_stuff:Vec<char> = self.stacks[from].split_off(idx);
        self.stacks[to].append(&mut moved_stuff);
    }
}

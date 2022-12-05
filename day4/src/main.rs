use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day4 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day4 FILE");
        return;
    }

    let ranges = parse_input(file_path);

    let total_overlaps:i32 = ranges.iter().map(|r| if is_total_overlap(&r[0], &r[1]) {1} else {0}).sum();
    let overlaps:i32 = ranges.iter().map(|r| if is_overlap(&r[0], &r[1]) {1} else {0}).sum();

    println!("part 1: {}", total_overlaps);
    println!("part 2: {}", overlaps);
}

fn is_total_overlap(left: &(i32, i32), right:&(i32, i32)) -> bool {
    return (left.0 >= right.0 && left.1 <= right.1) || (right.0 >= left.0 && right.1 <= left.1);
}

fn is_overlap(left: &(i32, i32), right:&(i32, i32)) -> bool {
    return (right.1 >= left.0 && right.0 <= left.0) || (left.1 >= right.0 && left.0 <= right.0);
}

fn parse_input(file_path:&String) -> Vec<Vec<(i32, i32)>> {

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = text.lines();

    let mut ranges:Vec<Vec<(i32,i32)>> = Vec::new();
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    for l in lines {
        let caps = re.captures(l).unwrap();
        let a = caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        let b = caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        let A = caps.get(3).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        let B = caps.get(4).map_or("", |m| m.as_str()).parse::<i32>().unwrap();

        let mut range:Vec<(i32,i32)> = Vec::new();
        range.push((a,b));
        range.push((A,B));

        ranges.push(range);
    }
    return ranges;
}

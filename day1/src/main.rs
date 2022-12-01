use std::env;
use std::fs;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day1 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day1 FILE");
        return;
    }

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("Text:\n{text}");

    let blocks = text.split("\n\n").collect::<Vec<&str>>();

    //println!("Elf Contents:\n{:?}", blocks);

    let mut elves: Vec<Vec<i32>> = Vec::new();
    for b in blocks {
        let lines = b.split("\n").collect::<Vec<&str>>();
        //println!("Lines: {:?}\n", lines);

        let mut elf: Vec<i32> = Vec::new();
        for l in lines {
            if !l.is_empty() {
              let value = l.parse::<i32>().unwrap();
              elf.push(value);
            }
        }

        elves.push(elf);
    }
    //println!("Elves: {:?}\n", elves);


    let cals = elves.into_iter().map(|elf| elf.into_iter().sum()).collect::<Vec<i32>>();
    let max_value = cals.iter().max();
    match max_value {
        Some(max) => println!( "Max Value: {}", max),
        None      => println!( "No max value"),
    }
}

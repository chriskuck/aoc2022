use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day3 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day3 FILE");
        return;
    }

    // prepare scoring struct
    //

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let scores: HashMap<_,_> = chars.chars().zip((1..53).into_iter()).into_iter().collect();
    //println!("{:?}", scores);

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("Text:\n{text}");

    let first_lines = text.lines();

    //println!("Lines:\n{:?}", lines);

    let mut sum = 0;
    for l in first_lines {
        //println!("Line: {:?}", l);
        let mut shared_char:char = '\0';
        let (one, two) = l.split_at(l.len()/2);
        //println!("{} - {}", one, two);
        for c in one.chars() {
            //println!("{}", c);
            if two.find(c) != None {
                //println!("found: {}",c);
                shared_char = c;
            }
        }
        //println!("{} : {}", shared_char, scores[&shared_char]);
        sum += scores[&shared_char];
    }

    println!("part 1:{}", sum);

    let mut groups: Vec<String> = Vec::new();

    for (i, l) in text.lines().enumerate() {
        let bucket = i / 3;
        if bucket+1 > groups.len() {
            groups.push(l.to_string());
        } else {
            groups[bucket] = format!("{}{}", groups[bucket], l);
        }
    }
    let second_lines = text.lines().collect::<Vec<&str>>();
    //println!("{:?}", groups);
    sum = 0;
    for (i,g) in groups.into_iter().enumerate() {
        //println!("Line: {:?}", l);
        let mut shared_char:char = '\0';
        let (one, two, three) = (second_lines[i*3+0], second_lines[i*3+1], second_lines[i*3+2]);
        //println!("{} - {}", one, two);
        for gc in g.chars() {
            if one.find(gc) == None { continue; }
            if two.find(gc) == None { continue; }
            if three.find(gc) == None { continue; }
            shared_char = gc;
        }
        //println!("{} : {}", shared_char, scores[&shared_char]);
        sum += scores[&shared_char];
    }
    println!("part 2: {}", sum);
}

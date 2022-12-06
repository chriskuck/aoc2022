use std::env;
use std::fs;
use std::path::Path;

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

    let messages = parse_input(file_path);

    for m in &messages { 
        if m.is_empty() { continue; }
        println!("msg: {} - pkt marker: {}", m, find_packet_marker(&m));
    }

    for m in &messages { 
        if m.is_empty() { continue; }
        println!("msg: {} - msg marker: {}", m, find_message_marker(&m));
    }
}


fn parse_input(file_path: &str) -> Vec<String> {

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return text.split("\n").map(|msg| String::from(msg)).collect::<Vec<String>>();
}

fn find_message_marker(msg: &String) -> usize {
    for i in 0..msg.len()-13 {
        if is_marker(&msg[i..i+14]) { return i+14; }
    }
    println!("{}", msg);
    panic!("Couldn't find the marker");
}

fn find_packet_marker(msg: &String) -> usize {
    for i in 0..msg.len()-3 {
        if is_marker(&msg[i..i+4]) { return i+4; }
    }
    println!("{}", msg);
    panic!("Couldn't find the marker");
}

fn is_marker(sample: &str) -> bool {
    let mut found: Vec<char> = Vec::new();
    for c in sample.chars() {
        if found.iter().find(|&&fc| fc == c) != None { return false; }
        found.push(c);
    }
    return true;
}

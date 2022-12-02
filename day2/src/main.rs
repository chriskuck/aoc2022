use std::env;
use std::fs;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day2 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day2 FILE");
        return;
    }

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("Text:\n{text}");
    let lines = text.split("\n").collect::<Vec<&str>>();

    //println!("Lines: {:?}", lines);
    let mut total_score = 0;
    for l in &lines {
        if !l.is_empty() {
            let srounds = l.split(" ").collect::<Vec<&str>>();
            let round = (map_abc_to_throw(srounds[0]), map_xyz_to_throw(srounds[1]));
            total_score += score_second_player(round);
        }
    }

    println!("part 1: {}", total_score);

    total_score = 0;
    for l in &lines {
        if !l.is_empty() {
            let srounds = l.split(" ").collect::<Vec<&str>>();
            let left = map_abc_to_throw(srounds[0]);
            let round = (left, map_wld_to_throw(left, map_xyz_to_wld(srounds[1])));
            //println!("{}({:?}) -> {}", l, &round, score_second_player(round));
            total_score += score_second_player(round);
        }
    }
    println!("part 2: {}", total_score);
}

fn map_wld_to_throw(left:Throw, wld: WLD) -> Throw {

    if wld == WLD::DRAW {
        return left;
    }

    return match (left, wld) {
        (Throw::ROCK, WLD::WIN) => Throw::PAPER,
        (Throw::ROCK, WLD::LOSE) => Throw::SCISSOR,
        (Throw::PAPER, WLD::WIN) => Throw::SCISSOR,
        (Throw::PAPER, WLD::LOSE) => Throw::ROCK,
        (Throw::SCISSOR, WLD::WIN) => Throw::ROCK,
        (Throw::SCISSOR, WLD::LOSE) => Throw::PAPER,
        _ => panic!("bad combo"),
    }
}

fn map_xyz_to_throw(xyz: &str) -> Throw {
  return match xyz {
      "X" => Throw::ROCK,
      "Y" => Throw::PAPER,
      "Z" => Throw::SCISSOR,
      _ => panic!("bad throw"),
  }
}

fn map_xyz_to_wld(xyz: &str) -> WLD {
  return match xyz {
      "X" => WLD::LOSE,
      "Y" => WLD::DRAW,
      "Z" => WLD::WIN,
      _ => panic!("bad throw"),
  }
}
fn map_abc_to_throw(abc: &str) -> Throw {
  return match abc {
      "A" => Throw::ROCK,
      "B" => Throw::PAPER,
      "C" => Throw::SCISSOR,
      _ => panic!("bad throw"),
  }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Throw {
    ROCK,
    PAPER,
    SCISSOR
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum WLD {
    WIN,
    LOSE,
    DRAW
}

fn score_second_player(round: (Throw, Throw)) -> i32 {
    let mut score = 0;
    score += throw_score(round.1);
    score += determine_winner(round)*3;
    return score;
}

fn throw_score(throw: Throw) -> i32 {
    return match throw {
        Throw::ROCK => 1,
        Throw::PAPER => 2,
        Throw::SCISSOR => 3,
    }
}

fn determine_winner(round: (Throw, Throw)) -> i32 {

    if round == (Throw::ROCK, Throw::SCISSOR)
       || round == (Throw::SCISSOR, Throw::PAPER)
       ||  round == (Throw::PAPER, Throw::ROCK) {
           return 0;
    } else if
        round == (Throw::SCISSOR, Throw::ROCK)
        || round == (Throw::PAPER, Throw::SCISSOR)
        || round == (Throw::ROCK, Throw::PAPER) {
        return 2;
    } else if round.0 == round.1 {
        return 1;
    }
    panic!("this wasn't expected");
}

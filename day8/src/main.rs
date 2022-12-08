use std::env;
use std::fs;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: day7 FILE");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("usage: day7 FILE");
        return;
    }

    let map = parse_input(file_path);

    let visible: Vec<(usize,usize)> = find_visible(&map);

    let part_1 = map.width()*2 + (map.height()-2)*2 + visible.len();
    println!("part 1: {}", part_1);

    let part_2:u32 = *map.scenic_scores().iter().max().unwrap();
    println!("part 2: {}", part_2);

}

fn parse_input(file_path: &str) -> Map {

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut rows = Vec::new();
    for text_row in text.lines() {
        let mut row = Vec::new();
        for cell in text_row.chars() {
            row.push(cell.to_digit(10).unwrap());
        }
        rows.push(row);
    }

    Map {
        rows: rows,
    }
}

fn find_visible(map: &Map) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for y in 1..map.rows[0].len()-1 {
        for x in 1..map.rows.len()-1 {
            if map.is_visible_right(x,y) || map.is_visible_left(x,y) || map.is_visible_up(x,y) || map.is_visible_down(x,y) {
                result.push((x,y));
            }
        }
    }
    return result;
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<u32>>,
}

impl Map {

    fn get_value(&self, x: usize, y:usize) -> u32 {
        self.rows[y][x]
    }

    fn get_row(&self, y:usize) -> Vec<u32> {
        self.rows[y].clone()
    }
    fn get_col(&self, x:usize) -> Vec<u32> {
        self.rows.iter().map(|row| row[x]).collect()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn is_visible_right(&self, x: usize, y:usize) -> bool {
        let max_visibility: u32 = *self.get_row(y)[0..x].iter().max().unwrap();
        self.get_value(x,y) > max_visibility
    }

    fn is_visible_left(&self,x: usize, y:usize) -> bool {
        let max_visibility: u32 = *self.get_row(y)[x+1..self.width()].iter().max().unwrap();
        self.get_value(x,y) > max_visibility
    }

    fn is_visible_up(&self,x: usize, y:usize) -> bool {
        let max_visibility: u32 = *self.get_col(x)[y+1..self.height()].iter().max().unwrap();
        self.get_value(x,y) > max_visibility
    }

    fn is_visible_down(&self,x: usize, y:usize) -> bool {
        let max_visibility: u32 = *self.get_col(x)[0..y].iter().max().unwrap();
        self.get_value(x,y) > max_visibility
    }

    fn scenic_scores(&self) -> Vec<u32> {

        let mut result = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.push(self.scenic_score(x,y));
            }
        }
        result
    }

    fn scenic_score(&self,x: usize, y:usize) -> u32 {
        let row = self.get_row(y);
        let col = self.get_col(x);
        let val = self.get_value(x,y);

        let right = &row[x+1..row.len()].to_vec();
        let mut left = Vec::new();
        for le in row[0..x].iter().rev() {
            left.push(le.clone());
        }
        let mut up = Vec::new();
        for u in col[0..y].iter().rev() {
            up.push(u.clone());
        }
        let down = &col[y+1..row.len()].to_vec();

        self.count_tree(right, val)*self.count_tree(&left, val)*self.count_tree(&up, val)*self.count_tree(down, val)
    }

    fn count_tree(&self, list:&Vec<u32>, val:u32) -> u32 {
        let mut result: u32 = 0;
        for v in list {
            if *v >= val { return result+1; }
            else { result += 1; }
        }
        result
    }
}

use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

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

    let console_output = parse_input(file_path);

    let file_system = parse_commands(console_output);

    let part_1: usize = file_system.root_dir_sizes().into_iter().filter(|(_name, size)| *size <= 100000).map(|(_, size)| size).sum();
    println!("part 1:{part_1}");

    let free: usize = 70000000-file_system.root_dir_sizes().into_iter().map(|(_,size)| size).max().unwrap();
    let size_needed = 30000000-free;
    let dir_sizes = file_system.root_dir_sizes();
    let smallest_files = dir_sizes.into_iter().map(|(_,size)| size).filter(|size| *size >= size_needed);
    let part_2 = smallest_files.min().unwrap();
    println!("part 2:{part_2}");
}


fn parse_input(file_path: &str) -> Vec<Eval> {

    let text = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let evals:Vec<Eval> = Vec::new();
    return text
        .split("\n")
        .fold(evals, |mut acc, line| {
            if line.starts_with("$") {
                acc.push({ Eval {
                    command: String::from(line),
                    output: Vec::new(),
                }});
            } else if !line.is_empty() {
                acc.last_mut().unwrap().output.push(String::from(line));
            }
            acc
        })
}


fn parse_commands(evals: Vec<Eval>) -> FileSystem {

    let mut file_system = build_file_system();

    for eval in evals {
        if eval.command.starts_with("$ cd") {

            let cd_fmt = Regex::new(r"\$ cd ([./a-zA-Z1-9]*)").unwrap();
            let dir_name = &cd_fmt.captures(&eval.command).unwrap()[1];
            match dir_name {
                ".." =>  {
                    file_system.go_up_dir();
                },
                "/" => {
                    file_system.go_to_root();
                },
                _ => {
                    file_system.change_to_local_dir(dir_name);
                },
            }
        }
        else if eval.command.starts_with("$ ls") {
            for line in eval.output {
                if line.starts_with("dir") {
                    let dir_fmt = Regex::new(r"dir (.*)").unwrap();
                    file_system.add_dir(&dir_fmt.captures(&line).unwrap()[1]);
                } else {
                    let file_fmt = Regex::new(r"(\d+) (.*)").unwrap();
                    let caps = file_fmt.captures(&line).unwrap();
                    file_system.add_file(&caps[2], caps[1].parse::<usize>().unwrap());
                }
            }
        }
    }
    return file_system;
}

#[derive(Debug)]
struct Eval {
    command: String,
    output: Vec<String>,
}

#[derive(Debug)]
struct FileSystem {
    cwd: usize,
    arena: Vec<Dir>,
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent_dir: Option<usize>,
    child_dirs: Vec<usize>,
    files: Vec<(String, usize)>,
}

fn build_file_system() -> FileSystem {

    let mut fs = FileSystem {
        arena: Vec::new(),
        cwd: 0,
    };

    let root = Dir {
        name: "root".to_string(),
        parent_dir: None,
        child_dirs: Vec::new(),
        files: Vec::new(),
    };
    fs.arena.push(root);
    return fs;
}

impl FileSystem {

    fn change_to_local_dir(&mut self, name:&str) {
        let mut child_dir_iter = self.arena[self.cwd].child_dirs.iter();
        let new_cwd = child_dir_iter.find(|&dir_id| self.arena[*dir_id].name == name).unwrap();
        self.cwd = *new_cwd;
    }

    fn add_dir(&mut self, child:&str) {
        let new_dir = Dir {
            name: String::from(child),
            parent_dir: Some(self.cwd),
            child_dirs: Vec::new(),
            files: Vec::new(),
        };
        self.arena.push(new_dir);
        let new_id = self.arena.len()-1;

        self.arena[self.cwd].child_dirs.push(new_id);
    }
    fn add_file(&mut self, name:&str, size:usize) {
        self.arena[self.cwd].files.push((String::from(name), size));
    }
    fn go_up_dir(&mut self) {
        self.cwd = self.arena[self.cwd].parent_dir.unwrap();
    }
    fn go_to_root(&mut self) {
        self.cwd = 0;
    }

    fn root_dir_sizes(&self) -> Vec<(&str, usize)> {
        let mut sizes:Vec<(&str, usize)> = Vec::new();
        for dir in self.arena.iter() {
            sizes.push((&dir.name, self.dir_size(dir)));
        }
        // traverse all directories summing as we go
        sizes
    }

    fn dir_size(&self, dir: &Dir) -> usize {
        let recursive_dir_size: usize = dir.child_dirs.iter().map(|d| self.dir_size(&self.arena[*d])).sum();
        let file_size: usize = dir.files.iter().map(|f| f.1).sum();
        recursive_dir_size + file_size
    }
}

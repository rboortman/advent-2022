use crate::{Assignment, Output};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: Vec::new(),
        }
    }

    fn add_file(&mut self, f: File, path: &VecDeque<String>) {
        if path.len() < 1 {
            self.files.push(f);
        } else {
            let mut cloned_path = path.clone();
            let next_dir = cloned_path.pop_front().unwrap();

            for dir in &mut self.subdirectories {
                if dir.name == next_dir {
                    dir.add_file(f.clone(), &cloned_path);
                }
            }
        }
    }

    fn add_directory(&mut self, d: Directory, path: &VecDeque<String>) {
        if path.len() < 1 {
            self.subdirectories.push(d);
        } else {
            let mut cloned_path = path.clone();
            let next_dir = cloned_path.pop_front().unwrap();

            for dir in &mut self.subdirectories {
                if dir.name == next_dir {
                    dir.add_directory(d.clone(), &cloned_path);
                }
            }
        }
    }

    fn get_total_size(&self) -> i32 {
        let mut size = 0;
        size += self.files.iter().fold(0, |acc, f| acc + f.size);
        size += self
            .subdirectories
            .iter()
            .fold(0, |acc, d| acc + d.get_total_size());
        size
    }

    fn find_dir_with_lower_size(&self, size: i32) -> Vec<i32> {
        let mut sizes = Vec::new();
        let self_size = self.get_total_size();
        if self_size < size {
            sizes.push(self_size);
        }

        for d in self.subdirectories.clone() {
            sizes.append(&mut d.find_dir_with_lower_size(size));
        }

        sizes
    }

    fn get_all_dir_sizes(&self) -> Vec<(String, i32)> {
        let mut result = Vec::new();
        result.push((self.name.clone(), self.get_total_size()));
        for d in self.subdirectories.clone() {
            result.append(&mut d.get_all_dir_sizes());
        }
        result
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: String, size: i32) -> File {
        File { name, size }
    }
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
pub struct Operation {
    input: Command,
    output: Vec<String>,
}

impl Operation {
    fn new(cmd: Command) -> Operation {
        Operation {
            input: cmd,
            output: Vec::new(),
        }
    }

    fn add_output(&mut self, output: String) {
        self.output.push(output);
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Directory;
    type Output = Output;

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
        let mut operations = VecDeque::new();
        let mut current_operation = Operation::new(Command::Cd(String::from("/")));

        for line in input.lines().skip(1) {
            let words: Vec<&str> = line.split(' ').collect();
            match words.get(0) {
                Some(&"$") => match words.get(1) {
                    Some(&"ls") => {
                        operations.push_back(current_operation);
                        current_operation = Operation::new(Command::Ls)
                    }
                    Some(&"cd") => {
                        operations.push_back(current_operation);
                        current_operation =
                            Operation::new(Command::Cd(words.get(2).unwrap().to_string()))
                    }
                    _ => panic!("Something went wrong when parsing {}", line),
                },
                _ => current_operation.add_output(line.to_string()),
            }
        }
        operations.push_back(current_operation);
        operations.remove(0);

        let mut root = Directory::new(String::from("/"));
        let mut current_dir: VecDeque<String> = VecDeque::new();

        for op in operations {
            match op.input {
                Command::Ls => {
                    for line in op.output {
                        let words: Vec<&str> = line.split(' ').collect();
                        let name = words.get(1).unwrap().to_string();
                        match words.get(0) {
                            Some(&"dir") => {
                                let new_dir = Directory::new(name);
                                root.add_directory(new_dir, &current_dir);
                                // self.add_directory(new_dir)
                            }
                            _ => {
                                let size: i32 = words.get(0).unwrap().parse().unwrap();
                                root.add_file(File::new(name, size), &current_dir);
                            }
                        }
                    }
                }
                Command::Cd(name) => match name.as_str() {
                    ".." => {
                        current_dir.pop_back();
                    }
                    _ => {
                        current_dir.push_back(name);
                    }
                },
            }
        }

        Some(root)
    }

    fn silver(&self, root: &Self::Input) -> Option<Self::Output> {
        Some(
            root.find_dir_with_lower_size(100000)
                .into_iter()
                .fold(0, |acc, s| acc + s)
                .into(),
        )
    }

    fn gold(&self, root: &Self::Input) -> Option<Self::Output> {
        let max_size = 70_000_000;
        let to_free = 30_000_000;
        let total_used = root.get_total_size();
        let minimal_removal = total_used - max_size + to_free;
        let mut sizes = root.get_all_dir_sizes();

        sizes.sort_by(|a, b| a.1.cmp(&b.1));
        let to_remove = sizes
            .into_iter()
            .filter(|(_, s)| s >= &minimal_removal)
            .next()
            .unwrap();

        println!("{:?}", to_remove);
        Some((to_remove.1).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 95437)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 24933642)
    }
}

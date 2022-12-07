use crate::{Assignment, Output};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
    parent: Box<Option<Directory>>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: Vec::new(),
            parent: Box::new(None),
        }
    }

    fn add_file(&mut self, f: File) {
        self.files.push(f);
    }

    fn add_directory(&mut self, d: Directory) {
        self.subdirectories.push(d);
    }

    fn add_parent(&mut self, d: Directory) {
        self.parent = Box::new(Some(d));
    }

    fn get_parent(&self) -> Option<Directory> {
        *self.parent.clone()
    }

    fn get_root_dir(&self) -> &Directory {
        match *self.parent.clone() {
            Some(_) => self.get_root_dir(),
            None => self,
        }
    }

    fn has_dir(&self, name: String) -> Option<Directory> {
        for dir in self.subdirectories.clone() {
            if dir.name == name {
                return Some(dir);
            }
        }
        None
    }

    fn parse_operations(&mut self, operations: &mut VecDeque<Operation>) -> &mut Directory {
        let mut op = operations.pop_front().unwrap();

        match op.input {
            Command::Cd(name) => {
                let mut dir = if name == String::from("..") {
                    self.get_parent().unwrap()
                } else {
                    self.has_dir(name).unwrap()
                };
                dir.parse_operations(operations);
            }
            Command::Ls => {
                for line in op.output {
                    let words: Vec<&str> = line.split(' ').collect();
                    match words.get(0) {
                        Some(&"dir") => {
                            let mut new_dir = Directory::new(words.get(1).unwrap().to_string());
                            new_dir.add_parent(self.clone());
                            self.add_directory(new_dir)
                        }
                        _ => {
                            let size: i32 = words.get(0).unwrap().parse().unwrap();
                            self.add_file(File::new(words.get(1).unwrap().to_string(), size));
                        }
                    }
                }
                println!("Test");
            }
        }

        self
    }
}

#[derive(Debug, Clone)]
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

        operations.remove(0);

        println!("{:?}", operations);

        let mut root = Directory::new(String::from("/"));
        root.parse_operations(&mut operations);
        println!("{:?}", root);
        Some(root)

        // for line in input.lines() {
        //     let words: Vec<&str> = line.split(' ').collect();
        //     match words.get(0) {
        //         Some(&"$") => match words.get(1) {
        //             Some(&"ls") => continue,
        //             Some(&"cd") => {
        //                 let mut dir = current_dir
        //                     .has_dir(words.get(2).unwrap().to_string())
        //                     .unwrap();
        //             }
        //             _ => panic!("Had a problem parsing '{}'", line),
        //         },
        //     }
        // }

        // Some(root)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some((-1).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some((-1).into())
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
        assert_eq!(result, -1)
    }
}

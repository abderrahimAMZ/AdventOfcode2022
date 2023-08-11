use crate::day7::Command::{
    ChangeDirectory, DirectoryContentDirectory, DirectoryContentFile, ListDirectory,
};
use std::str::Lines;

//TODO:: create data structure for every type of data
#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}
impl File {
    fn new(name: String, size: i32) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    prev: Option<&'a Directory<'a>>,
    next: Vec<Option<&'a Directory<'a>>>,
    files: Vec<File>,
}
impl<'a> Directory<'a> {
    fn new(name: String) -> Self {
        Self {
            name: name,
            prev: None,
            next: vec![],
            files: vec![],
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
    fn add_directory(&mut self, dir: &'a Directory) {
        self.next.push(Some(dir))
    }
}
#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory(String),
    DirectoryContentFile(File),
    DirectoryContentDirectory(Directory<'a>),
    ListDirectory,
}
//TODO:: implement a tree for the directory data type using the commands (input data)
fn create_command(input:&str) ->Vec<Command>{
    input
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|line: Vec<&str>| {
            if *line.get(0).unwrap() != "$" {
                if *line.get(0).unwrap() != "dir" {
                    DirectoryContentFile(File::new(
                        line.get(1).expect("idk").to_string(),
                        line.get(0).expect("idk").parse::<i32>().unwrap(),
                    ))
                } else {
                    DirectoryContentDirectory(Directory::new(
                        line.get(1).expect("something").to_string(),
                    ))
                }
            } else if *line.get(1).unwrap() != "cd" {
                ListDirectory
            } else {
                ChangeDirectory(line.get(2).expect("chihaja").to_string())
            }
        }).collect()
}
pub fn create_tree(input: &str) {
    let commands = create_command(input);
 //   let head = construct_tree(commands);
    println!("{:?}",commands);
}
/*
fn construct_tree(commands: Vec<Command>) {
   for command in commands {
       match {
           ChangeDirectory(dir) => handleChangingDirectory(),
           DirectoryContentDirectory(Directory) => (),
           DirectoryContentFile(File)=> (),
           ListDirectory =>()
       };
   }
}
*/
//TODO:: walk through the tree to calculate folder

//TODO:: filter sizes that are less then 10000 and return the result

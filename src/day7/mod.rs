use crate::day7::Command::{
    ChangeDirectory, DirectoryContentDirectory, DirectoryContentFile, ListDirectory,
};
use std::str::Lines;
use std::{cell::RefCell, rc::Rc};
use std::cmp::max;

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
struct Directory {
    name: String,
    prev: Option<DirectoryRef>,
    next: Vec<Option<DirectoryRef>>,
    files: Vec<File>,
}
type DirectoryRef = Rc<RefCell<Directory>>;

impl Directory {
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
    fn add_directory(&mut self, dir: DirectoryRef) {
        self.next.push(Some(dir));
    }
    fn files_size(&self) -> i32 {
        self.files.iter().map(|file| file.size).sum::<i32>()
    }
}
fn find_child(parent: DirectoryRef, dir_name: String) -> Option<DirectoryRef> {
    parent
        .borrow()
        .next
        .iter()
        .find(|child| child.as_ref().unwrap().borrow().name == dir_name)
        .unwrap()
        .clone()
}
#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    DirectoryContentFile(File),
    DirectoryContentDirectory(Directory),
    ListDirectory,
}
//TODO:: implement a tree for the directory data type using the commands (input data)
fn create_command(input: &str) -> Vec<Command> {
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
        })
        .collect()
}
pub fn create_tree_day7(input: &str) -> i32 {
    let commands = create_command(input);
    let tree_head = construct_tree(commands);

    let mut sizes:Vec<i32> = vec![];
    let head_size = calculate_sizes(tree_head,&mut sizes);
    let result = filter_sizes(sizes,head_size);
    result
}

fn construct_tree(commands: Vec<Command>) -> Rc<RefCell<Directory>> {
    let mut head = Rc::new(RefCell::new(Directory::new("/".to_string())));
    let mut current_directory = head.clone();
    for command in commands {
        match command {
            ChangeDirectory(dir) => match dir.as_str() {
                "/" => (),
                ".." => {
                    let parent = current_directory.borrow().prev.clone().unwrap();
                    current_directory = parent;
                }
                _ => {
                    let child = find_child(current_directory, dir).unwrap().clone();
                    current_directory = child;
                }
            },

            DirectoryContentDirectory(mut Directory) => {
                Directory.prev = Some(current_directory.clone());
                current_directory
                    .borrow_mut()
                    .add_directory(Rc::new(RefCell::new(Directory)))
            }
            DirectoryContentFile(File) => current_directory.borrow_mut().add_file(File),
            ListDirectory => (),
        };
    }
    head
}
//TODO:: walk through the tree to calculate folder
fn calculate_sizes(head: DirectoryRef,sizes: &mut Vec<i32>) -> i32 {
    let size = head.borrow().files_size()
        + head
        .borrow()
        .next
        .iter()
        .map(|dir| calculate_sizes(dir.as_ref().unwrap().clone(),sizes))
        .sum::<i32>();
    sizes.push(size.clone());
    size
}
//TODO:: filter sizes that are less then 10000 and return the result
fn filter_sizes(mut sizes : Vec<i32>,head_size : i32)->i32 {

    let total_system_size = 70_000_000;
    let total_free_space = 30_000_000;

    println!("head size {}",head_size);
    let desired_free_space = total_free_space - (total_system_size - head_size);
    println!(" desired free space is {}",desired_free_space);
    let result = sizes.iter()
        .filter(|&&size| size >= desired_free_space)
        .map(|&size| size)
        .min()
        .unwrap();
    /*
    let mut less_than_free_space:Vec<i32> = sizes.into_iter().filter(|size| *size <= desired_free_space).collect();
    less_than_free_space.sort_by(|a,b| a.cmp(b));
    let mut less_than_free_space= less_than_free_space.iter();
    let mut result = 0;
    while result < desired_free_space {
        result = result + less_than_free_space.next().unwrap();
    }

     */
   result
}
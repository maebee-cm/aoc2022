use std::{rc::Rc, cell::RefCell, fmt::Debug, borrow::BorrowMut};

const INPUT: &str = include_str!("../inputs/day7.txt");

pub fn solve() {
    let root = parse_input(INPUT);
    part_one(&root);
    part_two(&root);
}

type DirHandle = Rc<RefCell<Dir>>;

struct Dir {
    name: String,
    size: usize,
    contents: Vec<FileSystemItem>,
    parent: Option<DirHandle>
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent = match &self.parent{
            Some(parent) => parent.borrow().name.clone(),
            None => "None".to_string()
        };
        f.debug_struct("Dir")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("contents", &self.contents)
            .field("parent", &parent)
            .finish()
    }
}

impl Dir {
    pub fn new(name: String, parent: Option<DirHandle>) -> Dir {
        Dir {
            name,
            size: 0,
            contents: Vec::new(),
            parent
        }
    }

    pub fn increase_size(&mut self, increment: usize) {
        self.size += increment;
        if self.parent.is_some() {
            self.parent.as_ref().unwrap().borrow_mut().as_ref().borrow_mut().increase_size(increment);
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

impl File {
    pub fn new(name: String, size: usize) -> File {
        File {
            name,
            size
        }
    }
}

#[derive(Debug)]
enum FileSystemItem {
    Dir(DirHandle),
    File(File)
}

fn parse_input(input: &str) -> DirHandle {
    let mut lines_iter = input.lines().peekable();
    let first_line = lines_iter.next()
        .expect("input doesn't have any lines");

    let mut current_dir;
    if let Some(base_dir) = first_line.strip_prefix("$ cd ") {
        current_dir = Rc::new(RefCell::new(Dir::new(base_dir.to_string(), None)));
    }
    else {
        panic!("No base directory as the first line in the input")
    }
    let root = current_dir.clone();

    while let Some(line) = lines_iter.next() {
        match &line[0..4] {
            "$ cd" => {
                match &line[5..] {
                    ".." => {
                        let parent = current_dir.borrow().parent.as_ref().unwrap().clone();
                        current_dir = parent;
                    }
                    _ => {
                        let new_dir_name = line[5..].to_string();
                        let mut new_dir = None;
                        for child in &current_dir.borrow().contents {
                            if let FileSystemItem::Dir(dir) = child {
                                if dir.borrow().name == new_dir_name {
                                    new_dir = Some(dir.clone());
                                    break;
                                }
                            }
                        }
                        if new_dir.is_some() {
                            current_dir = new_dir.unwrap();
                        }
                    }
                }
            }
            "$ ls" => {
                while let Some(&line) = lines_iter.peek() {
                    if line.starts_with('$') {
                        break;
                    }
                    let line = lines_iter.next().unwrap();

                    // directory listing
                    if line.starts_with("dir") {
                        let new_dir = Rc::new(RefCell::new(Dir::new(line[4..].to_string(), Some(current_dir.clone()))));
                        let current_dir_mut = current_dir.borrow_mut();
                        current_dir_mut.as_ref().borrow_mut().contents.push(FileSystemItem::Dir(new_dir));
                    }
                    // file listing
                    else {
                        let mut file_info = line.split_whitespace();
                        let (size, name) = (file_info.next().unwrap(), file_info.next().unwrap());
                        let size = size.parse().unwrap();
                        let file = File::new(name.to_string(), size);
                        let current_dir_mut = current_dir.borrow_mut();
                        current_dir_mut.as_ref().borrow_mut().contents.push(FileSystemItem::File(file));
                        current_dir_mut.as_ref().borrow_mut().increase_size(size);
                    }
                }
            },
            _ => unreachable!()
        }
    }

    root
}

fn part_one(root: &DirHandle) {
    println!("total size of all directiories over 100000: {}", part_one_recursive(&root));
}

fn part_one_recursive(root: &DirHandle) -> usize {
    let mut ret = 0;
    if root.borrow().size <= 100000 {
        ret += root.borrow().size;
    }

    for child in &root.borrow().contents {
        if let FileSystemItem::Dir(dir) = child {
            ret += part_one_recursive(&dir);
        }
    }
    ret
}

fn part_two(root: &DirHandle) {
    const DISK_SPACE_MAX: usize = 70000000;
    const DISK_SPACE_NEEDED: usize = 30000000;

    let minimum_dir_size = DISK_SPACE_NEEDED - (DISK_SPACE_MAX - root.borrow().size);
    println!("Smallest folder to delete size: {}", part_two_recursive(&root, minimum_dir_size))
}

fn part_two_recursive(root: &DirHandle, minimum_dir_size: usize) -> usize {
    let mut smallest = usize::MAX;

    for child in &root.borrow().contents {
        if let FileSystemItem::Dir(dir) = child {
            if dir.borrow().size >= minimum_dir_size && dir.borrow().size < smallest {
                smallest = dir.borrow().size;
            }

            smallest = usize::min(smallest, part_two_recursive(dir, minimum_dir_size));
        }
    }

    smallest
}
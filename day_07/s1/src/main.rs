use std::collections::HashMap;

fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

#[derive(Debug, PartialEq)]
struct Tree {
    directories: Vec<Directory>,
    links: Vec<Link>,
}

impl Tree {
    fn new() -> Self {
        Self {
            directories: Vec::new(),
            links: Vec::new(),
        }
    }

    fn find_parent(&self, parent: &str) -> String {
        let link = self
            .links
            .iter()
            .find(|&o| o.1 == *parent.to_string())
            .unwrap();
        link.0.clone()
    }

    fn add_file(&mut self, dir: &str, file: File) {
        let dir_index = self.get_dir_index(dir);
        self.directories[dir_index].files.push(file);
    }

    fn get_max_level(&self) -> usize {
        todo!();
    }

    fn get_sub_directories(&self, dir: &str) -> Vec<&Directory> {
        self.links
            .iter()
            .filter(|o| o.0 == *dir.to_string())
            .map(|o| self.get_dir(&o.1))
            .collect()
    }

    fn get_dir_index(&self, dir: &str) -> usize {
        self.directories
            .iter()
            .position(|o| o.name == *dir.to_string())
            .unwrap()
    }

    fn get_dir(&self, dir: &str) -> &Directory {
        let dir_index = self.get_dir_index(dir);
        &self.directories[dir_index]
    }
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    level: usize,
    files: Vec<File>,
    size: usize,
}

impl Directory {
    fn new(name: &str, level: usize) -> Self {
        Self {
            name: name.to_string(),
            level,
            files: Vec::new(),
            size: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Link(String, String);

impl Link {
    fn new(src: &str, dst: &str) -> Self {
        Self(src.to_string(), dst.to_string())
    }
}

fn parse_line(tree: &mut Tree, input: &str, current_dir: &mut String, current_level: &mut usize) {
    let words: Vec<&str> = input.split(' ').collect();
    match words[0].trim() {
        "$" => match words[1].trim() {
            "cd" => match words[2].trim() {
                "/" => {
                    let dir = Directory::new("/", 0);
                    *current_dir = "/".to_string();
                    tree.directories.push(dir);
                }
                ".." => {
                    *current_dir = tree.find_parent(current_dir);
                    *current_level -= 1;
                }
                _ => {
                    *current_dir = words[2].trim().to_string();
                    *current_level += 1;
                }
            },
            "ls" => (),
            _ => unreachable!(),
        },
        "dir" => {
            let dst = words[1].trim();
            let dir = Directory::new(dst, *current_level + 1);
            tree.directories.push(dir);
            let link = Link::new(current_dir, dst);
            tree.links.push(link);
        }
        _ => {
            let file = File::new(words[1], words[0].parse().unwrap());
            tree.add_file(current_dir, file);
        }
    }
}

fn run(input: Vec<String>) -> usize {
    let mut tree = Tree::new();
    let mut current_dir = String::new();
    let mut current_level: usize = 0;
    for line in input {
        parse_line(&mut tree, &line, &mut current_dir, &mut current_level)
    }
    dbg!(&tree);
    todo!();
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Player score: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_link_find_parent() {
        let mut tree = Tree::new();
        let link1 = Link::new("/", "a");
        let link2 = Link::new("/", "b");
        let link3 = Link::new("a", "c");
        tree.links.push(link1);
        tree.links.push(link2);
        tree.links.push(link3);
        dbg!(&tree);
        assert_eq!(tree.find_parent("a"), "/".to_string());
        assert_eq!(tree.find_parent("b"), "/".to_string());
        assert_eq!(tree.find_parent("c"), "a".to_string());
    }

    #[test]
    fn test_directory_add_file() {
        let mut tree = Tree::new();
        let dir1 = Directory::new("/", 0);
        let dir2 = Directory::new("a", 1);
        let dir3 = Directory::new("b", 1);
        tree.directories.push(dir1);
        tree.directories.push(dir2);
        tree.directories.push(dir3);
        tree.add_file("a", File::new("truc", 12000));
        dbg!(&tree);
        assert_eq!(tree.directories[1].files[0], File::new("truc", 12000));
    }

    #[test]
    fn test_directory_get_dir() {
        let mut tree = Tree::new();
        let dir1 = Directory::new("/", 0);
        let dir2 = Directory::new("a", 1);
        let dir3 = Directory::new("b", 1);
        tree.directories.push(dir1);
        tree.directories.push(dir2);
        tree.directories.push(dir3);
        dbg!(&tree);
        assert_eq!(tree.get_dir("b"), &tree.directories[2]);
    }

    #[test]
    fn test_directory_get_sub_directories() {
        let mut tree = Tree::new();
        let dir1 = Directory::new("/", 0);
        let dir2 = Directory::new("a", 1);
        let dir3 = Directory::new("b", 1);
        let dir4 = Directory::new("c", 2);
        tree.directories.push(dir1);
        tree.directories.push(dir2);
        tree.directories.push(dir3);
        tree.directories.push(dir4);
        let link1 = Link::new("/", "a");
        let link2 = Link::new("/", "b");
        let link3 = Link::new("b", "c");
        tree.links.push(link1);
        tree.links.push(link2);
        tree.links.push(link3);
        dbg!(&tree);
        assert_eq!(
            tree.get_sub_directories("/"),
            vec![&tree.directories[1], &tree.directories[2]]
        );
        assert_eq!(tree.get_sub_directories("b"), vec![&tree.directories[3]]);
    }

    #[test]
    fn test_parse_line_root() {
        let mut tree = Tree::new();
        let mut current_dir = String::new();
        let mut current_level = 0;
        let input = "$ cd /";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);

        assert_eq!(
            tree.directories[0],
            Directory {
                name: "/".to_string(),
                level: 0,
                files: Vec::new(),
                size: 0
            }
        );
    }

    #[test]
    fn test_parse_line_dir() {
        let mut tree = Tree::new();
        let mut current_dir = String::new();
        let mut current_level = 0;
        let input = "$ cd /";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "dir a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);

        dbg!(&tree);
        assert_eq!(
            tree.directories[0],
            Directory {
                name: "/".to_string(),
                level: 0,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(
            tree.directories[1],
            Directory {
                name: "a".to_string(),
                level: 1,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(tree.links[0], Link("/".to_string(), "a".to_string()));
    }

    #[test]
    fn test_parse_line_cd_dir() {
        let mut tree = Tree::new();
        let mut current_dir = String::new();
        let mut current_level = 0;
        let input = "$ cd /";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "dir a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "$ cd a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);

        dbg!(&tree);
        assert_eq!(
            tree.directories[0],
            Directory {
                name: "/".to_string(),
                level: 0,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(
            tree.directories[1],
            Directory {
                name: "a".to_string(),
                level: 1,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(tree.links[0], Link("/".to_string(), "a".to_string()));
        assert_eq!(current_dir, "a".to_string());
        assert_eq!(current_level, 1);
    }

    #[test]
    fn test_parse_line_cd_dot_dot() {
        let mut tree = Tree::new();
        let mut current_dir = String::new();
        let mut current_level = 0;
        let input = "$ cd /";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "dir a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "$ cd a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "$ cd ..";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);

        dbg!(&tree);
        assert_eq!(
            tree.directories[0],
            Directory {
                name: "/".to_string(),
                level: 0,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(
            tree.directories[1],
            Directory {
                name: "a".to_string(),
                level: 1,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(tree.links[0], Link("/".to_string(), "a".to_string()));
        assert_eq!(current_dir, "/".to_string());
        assert_eq!(current_level, 0);
    }

    #[test]
    fn test_parse_line_file() {
        let mut tree = Tree::new();
        let mut current_dir = String::new();
        let mut current_level = 0;
        let input = "$ cd /";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "dir a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "$ cd a";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "dir b";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "$ cd ..";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "12000 my_file.txt";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);
        let input = "12500 my_file_2.txt";
        parse_line(&mut tree, input, &mut current_dir, &mut current_level);

        dbg!(&tree);
        assert_eq!(
            tree.directories[0],
            Directory {
                name: "/".to_string(),
                level: 0,
                files: vec![
                    File::new("my_file.txt", 12000),
                    File::new("my_file_2.txt", 12500)
                ],
                size: 0
            }
        );

        assert_eq!(
            tree.directories[1],
            Directory {
                name: "a".to_string(),
                level: 1,
                files: Vec::new(),
                size: 0
            }
        );

        assert_eq!(tree.links[0], Link("/".to_string(), "a".to_string()));
        assert_eq!(tree.links[1], Link("a".to_string(), "b".to_string()));
        assert_eq!(current_dir, "/".to_string());
        assert_eq!(current_level, 0);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 95437);
    }
}

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

#[derive(PartialEq, Debug)]
struct Elf {
    clean_section_start: u32,
    clean_section_end: u32,
}

impl Elf {
    fn new(clean_section_start: u32, clean_section_end: u32) -> Self {
        Self {
            clean_section_start,
            clean_section_end,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Elfpair {
    clean_section_start: u32,
    clean_section_end: u32,
}

impl Elfpair {
    fn new(clean_section_start: u32, clean_section_end: u32) -> Self {
        Self {
            clean_section_start,
            clean_section_end,
        }
    }
}

fn is_section_contained(elf: Elf, elfpair: Elfpair) -> bool {
    if elfpair.clean_section_start >= elf.clean_section_start
        && elfpair.clean_section_end <= elf.clean_section_end
    {
        return true;
    }

    if elf.clean_section_start >= elfpair.clean_section_start
        && elf.clean_section_end <= elfpair.clean_section_end
    {
        return true;
    }
    false
}

fn parse_line(line: &str) -> (Elf, Elfpair) {
    let elves: Vec<&str> = line.split(',').collect();
    let clean_sections: Vec<&str> = elves[0].split('-').collect();
    let elf = Elf::new(
        clean_sections[0].parse().unwrap(),
        clean_sections[1].parse().unwrap(),
    );

    let clean_sections: Vec<&str> = elves[1].split('-').collect();
    let elfpair = Elfpair::new(
        clean_sections[0].parse().unwrap(),
        clean_sections[1].parse().unwrap(),
    );

    (elf, elfpair)
}

fn run(input: Vec<String>) -> u32 {
    let mut output: u32 = 0;
    for line in input {
        dbg!(&line);
        let (elf, elfpair) = parse_line(&line);
        if is_section_contained(elf, elfpair) {
            dbg!("contained");
            output += 1;
        }
    }
    output
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
    fn test_is_section_contained_01() {
        let elf = Elf::new(1, 3);
        let elfpair = Elfpair::new(1, 2);
        assert_eq!(is_section_contained(elf, elfpair), true);
    }

    #[test]
    fn test_is_section_contained_02() {
        let elf = Elf::new(1, 3);
        let elfpair = Elfpair::new(1, 3);
        assert_eq!(is_section_contained(elf, elfpair), true);
    }

    #[test]
    fn test_is_section_contained_03() {
        let elf = Elf::new(1, 3);
        let elfpair = Elfpair::new(1, 4);
        assert_eq!(is_section_contained(elf, elfpair), true);
    }

    #[test]
    fn test_is_section_contained_04() {
        let elf = Elf::new(2, 4);
        let elfpair = Elfpair::new(1, 4);
        assert_eq!(is_section_contained(elf, elfpair), true);
    }

    #[test]
    fn test_is_section_contained_05() {
        let elf = Elf::new(2, 4);
        let elfpair = Elfpair::new(3, 4);
        assert_eq!(is_section_contained(elf, elfpair), true);
    }

    #[test]
    fn test_parse_line() {
        let elf = Elf::new(1, 2);
        let elfpair = Elfpair::new(2, 3);
        assert_eq!(parse_line("1-2,2-3"), (elf, elfpair));
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 2);
    }
}

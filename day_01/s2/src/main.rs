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

fn run(mut input: Vec<String>, nb_values: usize) -> i32 {
    let mut partial_sum = 0;
    let mut output = Vec::new();

    // Push an empty line at the end to mark and of file and simplify algorithm
    input.push("".to_string());

    for line in input {
        if line.is_empty() {
            output.push(partial_sum);
            partial_sum = 0;
        } else {
            partial_sum += line.parse::<i32>().unwrap();
        }
    }
    output.sort_by(|a, b| b.cmp(a));
    output.iter().take(nb_values).sum()
}

fn main() {
    const NB_VALUES: usize = 3;
    let input = parse_input(None);

    let answer = run(input, NB_VALUES);

    println!("Sum of the {} highest values: {}", NB_VALUES, answer);
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
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
                    1000
                    2000
                    3000

                    4000

                    5000
                    6000

                    7000
                    8000
                    9000

                    10000
                    "
        )));
        let answer = run(input, 3);
        assert_eq!(answer, 45000);
    }
}

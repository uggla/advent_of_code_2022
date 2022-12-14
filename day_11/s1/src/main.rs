use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, pair},
    *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add((Value, Value)),
    Mul((Value, Value)),
}

#[derive(Debug, PartialEq)]
enum Throw {
    True(u32),
    False(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Value {
    Old,
    N(u32),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    operation: Operation,
    divisible: u32,
    throws: (Throw, Throw),
    thrown_count: usize,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, op) = alt((tag("*"), tag("+")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, val) = alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u32.map(Value::N),
    ))(input)?;

    let op = match op {
        "*" => Operation::Mul((Value::Old, val)),
        "+" => Operation::Add((Value::Old, val)),
        _ => unreachable!(),
    };

    Ok((input, op))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // Ouch nom is really cool !
    let (input, (_, id)) = delimited(
        tag("Monkey"),
        pair(multispace1, nom::character::complete::u32),
        tag(":"),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), nom::character::complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, divisible) = nom::character::complete::u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, throw_val1) = nom::character::complete::u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, throw_val2) = nom::character::complete::u32(input)?;
    Ok((
        input,
        Monkey {
            id,
            items: VecDeque::from(items),
            operation,
            divisible,
            throws: (Throw::True(throw_val1), Throw::False(throw_val2)),
            thrown_count: 0,
        },
    ))
}

fn monkey_action(monkey: &mut Monkey) -> Vec<(u32, u32)> {
    let mut throws_items: Vec<(u32, u32)> = Vec::new();
    for _ in 0..monkey.items.len() {
        let item = monkey.items.pop_front().unwrap();
        let worry_level = match monkey.operation {
            Operation::Add(val) => match val {
                (Value::Old, Value::Old) => (item + item) / 3,
                (Value::Old, Value::N(val)) => (item + val) / 3,
                (Value::N(_), _) => unreachable!(),
            },
            Operation::Mul(val) => match val {
                (Value::Old, Value::Old) => (item * item) / 3,
                (Value::Old, Value::N(val)) => (item * val) / 3,
                (Value::N(_), _) => unreachable!(),
            },
        };

        if worry_level % monkey.divisible == 0 {
            match monkey.throws.0 {
                Throw::True(val) => {
                    throws_items.push((worry_level, val));
                    monkey.thrown_count += 1;
                }
                _ => unreachable!(),
            }
        } else {
            match monkey.throws.1 {
                Throw::False(val) => {
                    throws_items.push((worry_level, val));
                    monkey.thrown_count += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    throws_items
}

fn dispatch_throws_items(monkeys: &mut [Monkey], throws_items: Vec<(u32, u32)>) {
    for (item, monkey_index) in throws_items {
        monkeys[monkey_index as usize].items.push_back(item);
    }
}

fn run(input: String) -> usize {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkey)(&input).unwrap();
    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let throws_items = monkey_action(&mut monkeys[monkey_index]);
            dispatch_throws_items(&mut monkeys, throws_items);
        }
    }

    monkeys.sort_by_key(|monkey| monkey.thrown_count);
    dbg!(&monkeys);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.thrown_count)
        .product()
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
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
    fn test_parse_monkey() {
        let input = indoc!(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            "
        );
        let parsed = parse_monkey(input).unwrap();
        assert_eq!(
            parsed.1,
            Monkey {
                id: 0,
                items: VecDeque::from(vec![79, 98]),
                operation: Operation::Mul((Value::Old, Value::N(19))),
                divisible: 23,
                throws: (Throw::True(2), Throw::False(3)),
                thrown_count: 0,
            },
        );
    }

    #[test]
    fn test_monkey_action() {
        let input = indoc!(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            "
        );
        let mut parsed = parse_monkey(input).unwrap();
        let throws_items = monkey_action(&mut parsed.1);
        assert_eq!(parsed.1.items, []);
        assert_eq!(throws_items, [(500, 3,), (620, 3,)]);
        assert_eq!(parsed.1.thrown_count, 2);
    }

    #[test]
    fn test_run() {
        let input = read_input(Some(indoc!(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
                "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 10605);
    }
}

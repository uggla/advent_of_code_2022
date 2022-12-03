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

enum GameStatus {
    Lost,
    Drawn,
    Win,
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn parse_opponent_choice(choice: &str) -> Choice {
    match choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => unreachable!(),
    }
}

fn parse_player_choice(choice: &str) -> Choice {
    match choice {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => unreachable!(),
    }
}

fn parse_line(line: String) -> (Choice, Choice) {
    let choice: Vec<&str> = line.split(' ').collect();
    (
        parse_player_choice(choice[1]),
        parse_opponent_choice(choice[0]),
    )
}

fn game_result(player_choice: Choice, opponent_choice: Choice) -> GameStatus {
    match (player_choice, opponent_choice) {
        (Choice::Rock, Choice::Rock) => GameStatus::Drawn,
        (Choice::Rock, Choice::Paper) => GameStatus::Lost,
        (Choice::Rock, Choice::Scissors) => GameStatus::Win,
        (Choice::Paper, Choice::Rock) => GameStatus::Win,
        (Choice::Paper, Choice::Paper) => GameStatus::Drawn,
        (Choice::Paper, Choice::Scissors) => GameStatus::Lost,
        (Choice::Scissors, Choice::Rock) => GameStatus::Lost,
        (Choice::Scissors, Choice::Paper) => GameStatus::Win,
        (Choice::Scissors, Choice::Scissors) => GameStatus::Drawn,
    }
}

struct Player {
    score: u32,
}

impl Player {
    fn new() -> Self {
        Self { score: 0 }
    }

    fn calculate_score(&mut self, player_choice: Choice, opponent_choice: Choice) {
        let choice_score = match player_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        self.score += choice_score;

        let game_score = match game_result(player_choice, opponent_choice) {
            GameStatus::Win => 6,
            GameStatus::Drawn => 3,
            GameStatus::Lost => 0,
        };
        self.score += game_score;
    }
}

fn run(input: Vec<String>) -> u32 {
    let mut player = Player::new();

    for line in input {
        let (player_choice, opponent_choice) = parse_line(line);
        dbg!(&player_choice, &opponent_choice);
        player.calculate_score(player_choice, opponent_choice);
        dbg!(player.score);
    }
    player.score
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
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
                    A Y
                    B X
                    C Z
                    "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 15);
    }
}

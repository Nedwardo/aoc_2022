use std::fs;

fn main() {
    let filename = "data.txt";
    let file_contents = read_file(filename);
    let result = solve(file_contents);

    println!("Result: {:#?}", result);
}

fn read_file(filename: &str) -> String {
    let file_contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    return file_contents;
}

fn solve(file_contents: String) -> i32 {
    let score = file_contents
        .lines()
        .map(get_score_for_round)
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    return score;
}

fn unpack_round(line: &str) -> Vec<char> {
    let round = vec![line.chars().next().unwrap(), line.chars().last().unwrap()];
    return round;
}

fn get_score_for_round(round_as_line: &str) -> i32 {
    let round = unpack_round(round_as_line);

    let thrown_by_opponent = get_thrown(round[0]);
    let desired_state = get_state(round[1]);

    let thrown = get_what_to_throw(&thrown_by_opponent, &desired_state);
    let score = get_score_for_state(&desired_state) + get_score_for_thrown(&thrown);
    return score;
}

fn get_state(state_as_char: char) -> &'static State {
    match state_as_char {
        'X' => &State::Lose,
        'Y' => &State::Draw,
        'Z' => &State::Win,
        _ => panic!("Invalid state"),
    }
}

fn get_thrown(thrown_as_char: char) -> &'static Throwable {
    match thrown_as_char {
        'A' => &Throwable::Rock,
        'B' => &Throwable::Paper,
        'C' => &Throwable::Scissors,
        _ => panic!("Invalid thrown"),
    }
}

enum State {
    Win,
    Lose,
    Draw,
}

enum Throwable {
    Rock,
    Paper,
    Scissors,
}

fn get_score_for_state(state: &State) -> i32 {
    match state {
        State::Win => 6,
        State::Lose => 0,
        State::Draw => 3,
    }
}

fn get_score_for_thrown(thrown: &Throwable) -> i32 {
    match thrown {
        Throwable::Rock => 1,
        Throwable::Paper => 2,
        Throwable::Scissors => 3,
    }
}

fn get_what_to_throw<'a>(
    thrown_by_opponent: &'a Throwable,
    desired_state: &'a State,
) -> &'a Throwable {
    match (thrown_by_opponent, desired_state) {
        (Throwable::Rock, State::Win) => &Throwable::Paper,
        (Throwable::Rock, State::Lose) => &Throwable::Scissors,
        (Throwable::Rock, State::Draw) => &Throwable::Rock,
        (Throwable::Paper, State::Win) => &Throwable::Scissors,
        (Throwable::Paper, State::Lose) => &Throwable::Rock,
        (Throwable::Paper, State::Draw) => &Throwable::Paper,
        (Throwable::Scissors, State::Win) => &Throwable::Rock,
        (Throwable::Scissors, State::Lose) => &Throwable::Paper,
        (Throwable::Scissors, State::Draw) => &Throwable::Scissors,
    }
}

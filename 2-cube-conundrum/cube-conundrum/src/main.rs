use std::fs;

#[derive(Debug)]
struct Game {
    red: usize,
    green: usize,
    blue: usize,
}

fn get_result() -> usize {
    let content = fs::read_to_string("src/input-2.txt");
    let content_string = match content {
        Ok(text) => text,
        Err(_) => panic!("Could not read file"),
    };
    let content_string_vec: Vec<&str> = content_string.split("\n").collect();
    let game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_games = possible_games(&content_string_vec, &game);
    sum_possible_ids(&possible_games)
}

fn check_if_possible(game: &Game, color: &str, quantity: usize) -> bool {
    match color {
        "red" => game.red >= quantity,
        "green" => game.green >= quantity,
        "blue" => game.blue >= quantity,
        _ => false,
    }
}

fn remove_initial_string<'a>(text: &'a [&str]) -> Vec<&'a str> {
    text.iter()
        .map(|phrase| phrase.split(": ").last().unwrap_or(""))
        .collect()
}

fn possible_games<'a>(text: &'a [&str], initial_game: &Game) -> Vec<bool> {
    let text_without_string = remove_initial_string(text);
    text_without_string
        .iter()
        .map(|game| {
            game.replace(";", ",").split(", ").into_iter().all(|set| {
                let (color, quantity) = parse_set(set);
                check_if_possible(initial_game, color, quantity)
            })
        })
        .collect()
}

fn sum_possible_ids(games: &[bool]) -> usize {
    let mut sum = 0;
    for (index, game) in games.iter().enumerate() {
        if *game {
            sum = sum + index + 1;
        }
    }
    sum
}

fn parse_set(set: &str) -> (&str, usize) {
    let color = set.split(" ").last().unwrap_or("");
    let quantity = set
        .split(" ")
        .next()
        .unwrap_or("")
        .parse::<usize>()
        .unwrap_or(0);

    (color, quantity)
}

fn main() {
    println!("Result: {}", get_result());
}

#[test]
fn test_remove_initial_string() {
    let text = [
        "Game 1: 3 blue, 2 green, 6 red; 17 green, 4 red, 8 blue; 2 red, 1 green, 10 blue; 1 blue, 5 green",
        "Game 2: 9 red, 2 green; 5 red, 1 blue, 6 green; 3 green, 13 red, 1 blue; 3 red, 6 green; 1 blue, 14 red, 6 green",
        "Game 3: 6 red, 3 blue, 8 green; 6 blue, 12 green, 15 red; 3 blue, 18 green, 4 red"
    ];

    assert_eq!(remove_initial_string(&text), [
        "3 blue, 2 green, 6 red; 17 green, 4 red, 8 blue; 2 red, 1 green, 10 blue; 1 blue, 5 green",
        "9 red, 2 green; 5 red, 1 blue, 6 green; 3 green, 13 red, 1 blue; 3 red, 6 green; 1 blue, 14 red, 6 green",
        "6 red, 3 blue, 8 green; 6 blue, 12 green, 15 red; 3 blue, 18 green, 4 red"
    ])
}

#[test]
fn test_possible_games() {
    let text = [
        "3 blue, 2 green, 6 red; 5 green, 4 red, 8 blue; 2 red, 1 green, 5 blue; 1 blue, 5 green",
        "9 red, 2 green; 5 red, 1 blue, 6 green; 3 green, 13 red, 1 blue; 3 red, 6 green; 1 blue, 14 red, 6 green",
        "6 red, 3 blue, 8 green; 6 blue, 12 green, 15 red; 3 blue, 18 green, 4 red"
    ];

    let game = Game {
        red: 10,
        green: 10,
        blue: 10,
    };

    assert_eq!(possible_games(&text, &game), [true, false, false])
}

#[test]
fn test_parse_set() {
    let set1 = "3 blue";
    let set2 = "6 red";
    let set3 = "5 green";
    assert_eq!(parse_set(&set1), ("blue", 3));
    assert_eq!(parse_set(&set2), ("red", 6));
    assert_eq!(parse_set(&set3), ("green", 5));
}

#[test]
fn test_check_if_possible() {
    let game = Game {
        red: 10,
        green: 10,
        blue: 10,
    };

    assert_eq!(check_if_possible(&game, "red", 10), true);
    assert_eq!(check_if_possible(&game, "red", 11), false);
    assert_eq!(check_if_possible(&game, "green", 10), true);
    assert_eq!(check_if_possible(&game, "green", 11), false);
    assert_eq!(check_if_possible(&game, "blue", 10), true);
    assert_eq!(check_if_possible(&game, "blue", 11), false);
}

#[test]
fn test_sum_possible_ids() {
    let games = [true, false, false];
    let games2 = [true, true, false, true, false, true, true];
    assert_eq!(sum_possible_ids(&games), 1);
    assert_eq!(sum_possible_ids(&games2), 20);
}

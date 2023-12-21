use std::fs;

#[derive(Debug, Clone)]
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
    let possible_games = possible_games(&content_string_vec);
    total_power_cubes(&possible_games)
}

fn update_game_if_more_quantity(game: Game, color: &str, quantity: usize) -> Game {
    match color {
        "red" => {
            if game.red < quantity {
                return Game {
                    red: quantity,
                    ..game
                };
            }
            game
        }
        "green" => {
            if game.green < quantity {
                return Game {
                    green: quantity,
                    ..game
                };
            }
            game
        }
        "blue" => {
            if game.blue < quantity {
                return Game {
                    blue: quantity,
                    ..game
                };
            }
            game
        }
        _ => game,
    }
}

fn remove_initial_string<'a>(text: &'a [&str]) -> Vec<&'a str> {
    text.iter()
        .map(|phrase| phrase.split(": ").last().unwrap_or(""))
        .collect()
}

fn possible_games<'a>(text: &'a [&str]) -> Vec<Game> {
    let text_without_string = remove_initial_string(text);
    text_without_string
        .iter()
        .map(|game| {
            game.replace(";", ",").split(", ").into_iter().fold(
                Game {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, set| {
                    let (color, quantity) = parse_set(set);

                    update_game_if_more_quantity(acc.clone(), color, quantity)
                },
            )
        })
        .collect()
}

fn total_power_cubes(games: &[Game]) -> usize {
    let mut sum = 0;
    for game in games.iter() {
        sum = sum + ((*game).red * (*game).green * (*game).blue);
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

    let possible_games = possible_games(&text);
    assert_eq!(possible_games[0].red, 6);
    assert_eq!(possible_games[0].green, 5);
    assert_eq!(possible_games[0].blue, 8);
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

    assert_eq!(
        update_game_if_more_quantity(game.clone(), "red", 13).red,
        13
    );
}

#[test]
fn test_total_power_cubes() {
    let games = [
        Game {
            red: 3,
            green: 2,
            blue: 6,
        },
        Game {
            red: 9,
            green: 2,
            blue: 0,
        },
        Game {
            red: 6,
            green: 3,
            blue: 8,
        },
    ];
    assert_eq!(total_power_cubes(&games), 3 * 2 * 6 + 9 * 2 * 0 + 6 * 3 * 8);
}

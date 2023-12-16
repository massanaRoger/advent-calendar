use std::fs;

fn main() {
    let text_content = get_text_content();
    let total_sum = trebuchet(text_content);
    println!("Total sum is: {}", total_sum);
}

fn get_text_content() -> Vec<String> {
    let content = fs::read_to_string("input.txt");
    let content_string = match content {
        Ok(text) => text,
        Err(_) => panic!("Could not read file"),
    };
    content_string
        .split("\n")
        .map(|string| string.to_string())
        .filter(|string| !string.is_empty())
        .collect()
}

fn trebuchet(text_content: Vec<String>) -> i32 {
    text_content.iter().fold(0, |acc, b| {
        let converted_string = convert_string_number_to_number(b.clone());
        let char_vec: Vec<char> = converted_string
            .chars()
            .filter(|char| char.is_numeric())
            .collect();
        let first_number = char_vec.get(0).unwrap_or(&'0');
        let last_number = char_vec.get(char_vec.len() - 1).unwrap_or(&'0');
        acc + format!("{}{}", first_number, last_number)
            .parse::<i32>()
            .unwrap_or(0)
    })
}

fn convert_string_number_to_number(mut text: String) -> String {
    text = replace_string_with_number(text);
    println!("text: {}", text);
    text.chars()
        .filter(|char| char.is_numeric())
        .collect::<String>()
}

fn replace_string_with_number(text: String) -> String {
    let replacements = [
        ("three", "3"),
        ("eight", "8"),
        ("seven", "7"),
        ("six", "6"),
        ("five", "5"),
        ("four", "4"),
        ("two", "2"),
        ("one", "1"),
        ("nine", "9"),
    ];

    let mut new_string = "".to_string();

    for char in text.chars() {
        new_string.push(char);
        for &(word, number) in &replacements {
            if new_string.contains(word) {
                new_string = new_string.replace(word, number);
                new_string.push(word.chars().nth(word.len() - 1).unwrap_or('.'))
            }
        }
    }
    new_string
}

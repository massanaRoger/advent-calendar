fn main() {
    let text_content = get_text_content();
    let total_sum = trebuchet(text_content);
    println!("Total sum is: {}", total_sum);
}

fn get_text_content() -> Vec<&'static str> {
    vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
}

fn trebuchet(text_content: Vec<&str>) -> i32 {
    text_content.iter().fold(0, |acc, b| {
        let char_vec: Vec<char> = b.chars().filter(|char| char.is_numeric()).collect();
        let first_number = char_vec[0];
        let last_number = char_vec[char_vec.len() - 1];
        acc + format!("{}{}", first_number, last_number)
            .parse::<i32>()
            .unwrap_or(0)
    })
}

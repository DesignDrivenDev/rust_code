fn shortest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let input = "I am a good boy!";
    let shortest = shortest_word(input);
    println!("The shortest word is: {}", shortest);
}

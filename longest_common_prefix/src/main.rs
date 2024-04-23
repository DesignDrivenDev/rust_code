fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0]; // Take the first string as a reference

    // Iterate over characters of the first string
    for (i, &char) in first_string.as_bytes().iter().enumerate() {
        // Check if all other strings have the same character at position i
        if !strings.iter().all(|s| s.as_bytes().get(i) == Some(&char)) {
            return first_string[..i].to_string(); // Return the prefix up to position i
        }
    }

    first_string.to_string() // If all strings are identical, return the first string itself
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}

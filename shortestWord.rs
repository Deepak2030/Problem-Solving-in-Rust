fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let s1 = "the quick brown fox jumps over the lazy dog";
    let s2 = "";
    let s3 = "    ";

    match shortest_word(s1) {
        Some(word) => println!("The shortest word in '{}' is '{}'", s1, word),
        None => println!("There are no words in '{}'", s1),
    }

    match shortest_word(s2) {
        Some(word) => println!("The shortest word in '{}' is '{}'", s2, word),
        None => println!("There are no words in '{}'", s2),
    }

    match shortest_word(s3) {
        Some(word) => println!("The shortest word in '{}' is '{}'", s3, word),
        None => println!("There are no words in '{}'", s3),
    }
}

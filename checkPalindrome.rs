use std::io;

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    println!("Enter a string:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");

    let s = s.trim(); // Remove leading/trailing whitespace

    if is_palindrome(s) {
        println!("'{}' is a palindrome", s);
    } else {
        println!("'{}' is not a palindrome", s);
    }
}

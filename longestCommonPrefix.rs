fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first = strings[0].as_bytes();
    
    for (i, &ch) in first.iter().enumerate() {
        for str in &strings[1..] {
            let bytes = str.as_bytes();
            if i >= bytes.len() || bytes[i] != ch {
                return String::from_utf8_lossy(&first[..i]).to_string();
            }
        }
    }

    String::from_utf8_lossy(first).to_string()
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let prefix = longest_common_prefix(&strings);
    println!("The longest common prefix of {:?} is '{}'", strings, prefix);
}

fn find_shortest_word(s: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    for word in s.split_whitespace() {
        if shortest_word.is_none() || word.len() < shortest_word.unwrap().len() {
            shortest_word = Some(word);
        }
    }
    shortest_word
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest_word) = find_shortest_word(s) {
        println!("The shortest word in \"{}\" is \"{}\"", s, shortest_word);
    } else {
        println!("No words found in \"{}\"", s);
    }
}


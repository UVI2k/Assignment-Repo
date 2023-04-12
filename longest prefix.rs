fn longest_common_prefix(strs: &[&str]) -> String {
    let mut prefix = String::new();
    if strs.is_empty() {
        return prefix;
    }
    let mut chars = strs.iter().map(|s| s.chars());
    loop {
        let mut current_char = None;
        for c in chars.next().unwrap() {
            if current_char.is_none() {
                current_char = Some(c);
            } else if current_char.unwrap() != c {
                return prefix;
            }
        }
        if let Some(c) = current_char {
            prefix.push(c);
        } else {
            return prefix;
        }
    }
}

fn main() {
    let strs = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strs);
    println!("The longest common prefix is '{}'", prefix);
}

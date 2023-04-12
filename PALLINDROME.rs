fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); // convert to lowercase
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); // remove non-alphanumeric characters
    let rev_s = s.chars().rev().collect::<String>(); // reverse the string
    s == rev_s // check if the original string and the reversed string are equal
}

fn main() {
    let s1 = "A man a plan a canal Panama";
    let s2 = "not a palindrome";
    println!("{} is a palindrome: {}", s1, is_palindrome(s1));
    println!("{} is a palindrome: {}", s2, is_palindrome(s2));
}

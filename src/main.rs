use std::collections::VecDeque;

fn reverse_words(s: &str) -> String {
    let mut words = VecDeque::new();
    let mut word = String::new();

    for c in s.chars() {
        if c.is_whitespace() {
            if !word.is_empty() {
                words.push_front(word);
                word.clear();
            }
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        words.push_front(word);
    }

    words.into_iter().collect::<String>()
}

fn main() {
    let input = "snow dog sun";
    let output = reverse_words(input);
    println!("{}", output);
}
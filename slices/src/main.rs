fn main() {
    let words = String::from("Hello World");

    let first_idx = first_word(&words);
    println!("{}", first_idx);

    let first_idx = first_word2(&words);
    println!("{}", first_idx);

    let first_idx = first_word3(&words);
    println!("{}", first_idx);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return s;
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word는 5를 갖게 될 것입니다.

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
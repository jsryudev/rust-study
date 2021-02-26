fn main() {
    let mut s = String::from("hello");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
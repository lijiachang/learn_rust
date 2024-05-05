pub fn main() {
    let mut s = String::from("hello world");
    let word_index = first_world(&s);

    println!("{s}, first world={word_index}")
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    return &s[..]
}
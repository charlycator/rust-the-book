
fn main() {
    let s = String::from("hello world");
    let pos = first_word(&s);

    println!("{}", pos);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

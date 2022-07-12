fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    println!("the first word is: {}", word);
    let word = first_word(&my_string[..]);
    println!("the first word is: {}", word);
    let word = first_word(&my_string);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    println!("the first word is: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("the first word is: {}", word);
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word)
}

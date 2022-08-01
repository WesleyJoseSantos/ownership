fn string_handler() {
    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[..];

    let s2 = "hello world";

    println!("s: {}", s);
    println!("hello: {}", hello);
    println!("world: {}", world);

    println!("the first world in s is: {}", first_word(&s));
    println!("the first world in s2 is: {}", first_word(&s2));
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
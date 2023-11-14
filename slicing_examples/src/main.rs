fn main() {
    ex1();
}

fn ex1() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Word will get the value 5
    s.clear(); // This empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfuly use the value 5 with. Word is now totally invalid!
}

fn first_word(s: &String) -> {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn ex2() {
    let my_string = String::from("hello world");
    let word = first_word3(&my_string[0..6]);// First_word works on slices of Strings whether partial or whole
    let word = first_word3(&my_string[..]);// First_word works on slices of Strings whether partial or whole
    let word = first_word3(&my_string);// This works because references to Strings are equivalent to whole slices of Strings

    let my_string_literal = "hello world";
    let word = first_word3(&my_string_literal[0..6]);// First_word works on slices of string literals
    let word = first_word3(&my_string_literal[..]); // First_word works on slices of string literals
    let word = first_word3(my_string_literal);// This works because string literals are string slices
}


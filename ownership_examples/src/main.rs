fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
}

// EXAMPLE 1
fn ex1()
{ // s is not valid here, since it's not yet declared
    let s = "hello"; // This is a string literal
} // This scope is now over, and s is no longer valid


// EXAMPLE 2
fn ex2()
{
    let s = String::from("hello"); // String - Managed data allocated on heap

    let mut s2 = String::from("hello"); // String can be mutated, unlike string literal
    s2.push_str(", world!"); // Appends to String
    println!("{s2}");
}

// EXAMPLE 3
fn ex3(){
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so it is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);  // x would move into the function,
                    // but i32 is Copy, so it's okay to still use x afterwards
} // Here, x goes out of scope, then s. However, because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


// EXAMPLE 4
fn ex4(){
    let s1 = gives_ownership(); // gives_ownership moves into its return value in s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, 
                                       // which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
  // s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let mut some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling functions
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling functions
}

// EXAMPLE 5
fn ex5() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // Because we're passing s1 in but not returning, we cannot use s1 after this
    println!("The Length of {s2} is {len}.");
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// Example 5 pt. 2
fn ex5_2() {
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

// Example 6
fn ex6() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // This will fail because we're trying to modify a borrowed value
                                     // References are static by default. We'll need to use &mut
}

fn ex6_2() {
    let mut s = String::from("hello");
    change2(&mut s);

    let r1 = &mut s;
    let r2 = &mut s; // cannot reference value twice w/ mut
    let r3 = &s; // no issue
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
    
}

fn ex1()
{ // s is not valid here, since it's not yet declared
    let s = "hello"; // This is a string literal
} // This scope is now over, and s is no longer valid

fn ex2()
{
    let s = String::from("hello"); // String - Managed data allocated on heap

    let mut s2 = String::from("hello"); // String can be mutated, unlike string literal
    s2.push_str(", world!"); // Appends to String
    println!("{s2}");
}
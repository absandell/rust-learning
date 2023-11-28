fn main() {
    let v: Vec<i32> = Vec::new(); // Create a new, empty vector to hold types of i32
    let v2 = vec![1,2,3];
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1,2,3,4,5];
    let third: &i32 = &v4[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v5 = vec![100,32,57];
    for i in &v5 {
        println!("{i}");
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }
    // Vectors require all the same type of value. If we don't want this, we need to use an enum
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    {
        let v = vec![1,2,3,4];
        //do stuff with v
    }// <- v goes out of scope

    // String Literals are a wrapper around a Vector

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");// Appends "bar"

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // We cannot index strings directly in rust (s1[0] for instance) because this could return unexpected values

    for c in "hi".chars(){
        println!("{c}"); // prints "h i"
    }

    for b in "hi".bytes() {
        println!("{b}"); // prints "104 105"
    }

    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);// get() returns an <option>, so unwrap_or sets the score to 0 if no key entry
    for (key,value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50); // This overwrites the prior mapping
    scores.entry(String::from("Blue")).or_insert(75); // This would insert 75 if blue did not exist
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The value of Blue is: {score}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // Need to dereference because or_insert returns a mutable reference to the location
    }
    println!("{:?}", map);

}

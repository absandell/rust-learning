fn main() {
    let v: Vec<i32> = Vec::new(); // Create a new, empty vector to hold types of i32
    let v2 = vec![1,2,3]
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
}

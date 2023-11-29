/* STRUCTS */
struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/* DRIVER FUNCTIONS */
fn main() {
    //ex1();
    //ex2();
    //ex3();
    ex4();
}

fn ex1() { // Lots of duplicate code
    let number_list = vec![34,50,25,100,65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    let number_list = vec![102,34,6000,89,54,2,43,8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
}

fn ex2() { // No duplicate code
    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102,34,6000,89,54,2,43,8];
    let result = largest(&number_list);
    println!("The largest number is {result}");
}

fn ex3() { // No duplicate code
    let number_list = vec![34,50,25,100,65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

fn ex4() { // No duplicate code
    let number_list = vec![34,50,25,100,65];
    let result = find_largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest(&char_list);
    println!("The largest char is {result}");
}

fn ex5() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    // let wont_work = Point {x: 5, y: 4.0}; // Both need to be of same type, no mixtures
    let integer_and_foat = MixedPoint{x: 5, y: 4.0};
    println!("float.x = {}", float.x());
}

/* HELPER FUNCTIONS */

fn largest(list:&[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list:&[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // Use std::cmp::PartialOrd because not all <T> can be compared with binary operations like >
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


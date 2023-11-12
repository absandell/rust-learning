fn main() {

    //print_labeled_measurement(5, 'h');
    //five_plus_one();
    //return_example();
    //fizzbuzz();
    //bool_comp();
    //value_loop();
    //labeled_loop();
    //conditional_loop();
    //collection_loop();
    //concise_collection_loop();
    //rev_countdown();
    //temp_conversions();
    nth_fibonacci_driver();

}

// Outputs value of int passed in
fn another_function(x: i32) {
    println!("The value of argument is : {x}\n");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}\n");
}

fn five_plus_one() {
    let x = plus_one(five());
    another_function(x);
}

fn return_example() {
    let y = {
        let x = 3;
        x+ 1
    };

    another_function(y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn fizzbuzz() {
    let number = 3;

    if number % 4 == 0 {
        println!("Number is divisible by 4\n");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3\n");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2\n");
    } else {
        println!("Number is not divisible by 4, 3, or 2\n");
    }
}

fn bool_comp() {
    let condition = true;
    let number = if condition {5} else {6};
    another_function(number);
}

fn value_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    another_function(result);
}

fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}\n");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn collection_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}\n", a[index]);

        index += 1;
    }
}

fn concise_collection_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element\n}");
    }
}

fn rev_countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!\n");
}

fn temp_conversions() {
    let f = 100.94;
    let c = 38.3;
    println!("{} degrees Fahrenheit is {} degrees Celsisus\n", f, f_to_c(f));
    println!("{} degrees Celsius is {} degrees Fahrenheit\n", c, c_to_f(c));
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}
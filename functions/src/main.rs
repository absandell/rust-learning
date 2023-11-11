fn main() {

    print_labeled_measurement(5, 'h');
    five_plus_one();
    return_example();
    fizzbuzz();
    bool_comp();
    value_loop();

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
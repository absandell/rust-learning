use std::fmt;

enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method body would be defined here
    }
}

impl fmt::Display for UsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin { 
        Coin::Penny =>{
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
/* 
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
    else {
        count +=1;
    }

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!, state"),
        _ => count += 1,
    }
    */
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn roll_the_dice(dice_roll: &i32) -> i32 {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _ => reroll(), // used when we want a catch all but don't want to use the matched value
    }
}


fn add_fancy_hat() -> i32 {
    3
}
fn remove_fancy_hat() -> i32 {
    7
}
fn move_player(num_space: &i32) -> i32 {
    8
}
fn reroll() -> i32 {
    9
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    roll_the_dice(&dice_roll);

    let config_max = Some(3u8);
    if let Some(max) = config_max { // This is a clean option
        println!("The maximum is configured to be {max}");
    }
    match config_max { // This is a less clean option
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    

}



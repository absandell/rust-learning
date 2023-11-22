fn main() {
    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point (i32, i32, i32);

struct AlwaysEqual;

fn ex1() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Specifies that the rest of the fields should be copied from user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User { // Same as above, just using init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn ex2() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

fn ex3() {
    let subject = AlwaysEqual;
}


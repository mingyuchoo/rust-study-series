// similar to data type in Haskell
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// similar to data type using record syntax in Haskell
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // == email: email,       // field init shorthand syntax
        username, // == username: username, // field init shorthand syntax
        active: true,
        sign_in_count: 1,
    }
}

pub fn create_instance() {
    println!("-- create_instance()");

    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = build_user(String::from("newone@example.com"), String::from("newone"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    user1.email = String::from("anotheremail@example.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

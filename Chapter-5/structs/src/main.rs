
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main() {
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user1 email: {}", user1.email);
    println!("user1 username: {}", user1.username);
    println!("user1 sign_in_count: {}", user1.sign_in_count);
    println!("user1 active: {}", user1.active);
    println!("user2: {}", user2.email);
    println!("user3: {}", build_user("cooldude@gmail.com".to_string(), "cooldude1990".to_string()).email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

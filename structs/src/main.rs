fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("gaoutaman"),
        email: String::from("gaoutaman@rust.com"),
        sign_in_count: 1,
    };
    println!("User email is {email}", email = user1.email);
    user1.email = String::from("gaoutaman2@rust.com");
    println!("User email is now {email}", email = user1.email);

    let user2 = build_user(String::from("gman"), String::from("gman@rust.com"));
    println!("User 2 is {name}", name = user2.username);

    let user3 = User {
        email: String::from("user3@urst.com"),
        ..user2
    };
    println!(
        "User3 is same as user2 except for the email: {email}. Same name: {name}",
        email = user3.email,
        name = user3.username
    );

    let black = Colour(1, 2, 3);

    let Colour(x, y, z) = black;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Colour(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

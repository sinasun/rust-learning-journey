struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("sina"),
        email: String::from("sina@gmail.com"),
        sing_in_count: 1,
    };
    user1.email = String::from("sinasun");
    let user2 = build_user(String::from("nima@gmail.com"), String::from("nima"));
    let user3 = User {
        email: String::from("example@gmail.com"),
        ..user2
    };
    let username = user2.active;
    println!("user1: {username}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("Jabbslad"),
        email: String::from("none@none.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
}

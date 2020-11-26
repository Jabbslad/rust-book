#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("Jabbslad"),
        email: String::from("none@none.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("some@some.com");
    println!("{:?}", user1);

    let user1 = build_user(String::from("Jabbslad"), String::from("none@none.com"));
    println!("{:?}", user1);
    let user2 = User {
        username: String::from("User2"),
        email: String::from("user2@none.com"),
        ..user1
    };
    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    #[derive(Debug)]
    struct CoOrd(i32, i32);
    let coord = CoOrd(1, 2);

    println!("{:?}, {:?}, {:?}", black, origin, coord);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

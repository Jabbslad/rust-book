fn main() {
    let x = String::from("hello");
    takes_ownership(x);

    let y = 5;
    makes_copy(y);
    println!("hey its fine to us 'y' afterwards: {}", y);

    let _ = gives_ownership();
    let s2 = String::from("hello");
    println!("{:p}", &s2);
    let s3 = takes_and_gives_back(s2);
    println!("{:p}", &s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("{} - {}", s2, len);
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

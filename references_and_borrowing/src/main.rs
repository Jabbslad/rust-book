fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("{:p}", &s1);
    println!("length = {}", len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    let s3 = &mut s2;
    println!("{}", s3);

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    r3.push_str(" world!!!");
    println!("{}", r3);

    println!("no dangle: {}", no_dangle());

    let mut hello = String::from("Hello world!");
    let f = first(&hello);
    //hello.clear(); -- cannot do mutable borrow here.
    println!("first = '{}'", f);
}

fn first(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> usize {
    println!("{:p}", s);
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world!");
}

fn no_dangle() -> String {
    String::from("hello!!!")
}

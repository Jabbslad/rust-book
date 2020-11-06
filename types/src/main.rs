fn main() {
    let tup: (i32, i32, char) = (1, 3, 'c');
    let (_, _, c) = tup;
    println!("{}, {}", c, tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
}

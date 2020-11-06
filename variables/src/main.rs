fn main() {
    const MAX: u32 = 100_100;
    println!("{}", MAX);

    let mut non_shadow = 0;
    println!("{:p}", &non_shadow);
    non_shadow = 1;
    println!("{:p}", &non_shadow);

    let shadow = 0;
    println!("{:p}", &shadow);
    let shadow = 1;
    println!("{:p}", &shadow);
}

fn main() {
    println!(" 6F = {:>6.2}C", f_to_c(6.0));
    println!("41C = {:>6.2}F", c_to_f(41.0));
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f32) -> f32 {
    (c / 5.0) * 9.0 + 32.0
}

fn main() {
    let number = 3;

    if number < 5 {
        println!("Hello, world!");println!("condition was true");
    } else {
        println!("Condition was false");
    }

    println!("condition was {}", number < 5);

    let mut counter = 1;
    let result = loop {
        println!("again!");
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result = {}", result);

    while counter < 20 {
        println!("again: {}", counter);
        counter += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for n in a.iter() {
        print!("{} ", n);
    }

    println!();

    for n in (1..5).rev() {
        println!("{}", n);
    }
    println!("Lift-off!");
}

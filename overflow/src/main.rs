fn main() {
    println!("Hello, world!");
    let mut x: u8 = 0;
    println!("x is: {}", x);

    // Make x a `u8` type

    if true {
        println!("Hello!");
    }

    // Add something to x
    x += 100;
    println!("x is: {}", x);

    // Add something else to x, which will
    // overflow past 255
    x += 200;
    println!("x is: {}", x);

    // At each step, print the value of x
    // See what happens in debug and release mode
}

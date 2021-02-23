fn main() {
    println!("Hello, world!");

    for i in 1..=100 {
        let fb = fizzbuzz(i);
        println!("{} => {}", i, fb);
    }
}

fn fizzbuzz(x: u32) -> String {
    let div_3 = (x % 3) == 0;
    let div_5 = (x % 5) == 0;

    // match x {
    //     j if ((j % 3) == 0) && ((j % 5) == 0) => String::from("FizzBuzz"),
    //     j if ((j % 3) == 0) => String::from("Fizz"),
    //     j if ((j % 5) == 0) => String::from("Buzz"),
    //     j => {
    //         format!("{}", j)
    //     }
    // }

    let b = String::from("bob");

    // if div_3 {
    //     String::from("bob")
    // } else {
    //     10
    // };

    match (div_3, div_5) {
        (true, true) => {
            String::from("FizzBuzz")
        }
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (false, false) => format!("{}", x),
    }
}


#[test]
fn spot_check() {
    assert_eq!("Fizz", fizzbuzz(3));
    assert_eq!("4", fizzbuzz(4));
    assert_eq!("Buzz", fizzbuzz(5));
    assert_eq!("FizzBuzz", fizzbuzz(15));
}

#[test]
fn spot_check2() {
    assert_eq!("Fizz", fizzbuzz(3));
    assert_eq!("4", fizzbuzz(4));
    assert_eq!("Buzz", fizzbuzz(5));
    assert_eq!("FizzBuzz", fizzbuzz(15));
}

#[test]
fn spot_check3() {
    assert_eq!("Fizz", fizzbuzz(3));
    assert_eq!("4", fizzbuzz(4));
    assert_eq!("Buzz", fizzbuzz(5));
    assert_eq!("FizzBuzz", fizzbuzz(15));
}

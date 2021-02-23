

fn main() {
    let x = "1234";

    let a: u32 = x.parse().unwrap();
    println!("a: {}", a);

    // problem:
    let b = x.parse::<u64>().unwrap();

    let input = "#$1234.&&";
    // let input = "1234";

//  vvvvvv         - Begin a pattern binding
//         vvvvvvv - pattern binding
//                   vvvvvvvvvvvvvvvvvvvv - input
    if let Ok(num) = input.parse::<u64>() {
        println!("num: {}", num);
    } else {
        println!("Bad parse!");
    }

//  vvvvv - begin a pattern binding
//        vvvvvvvvvvvvvvvvvvvv - input
    match input.parse::<u64>() {
//      vvvvvvv - pattern binding
        Ok(num) => println!("num: {}", num),
        Err(e) => eprintln!("err: {:?}", e),
    }

}

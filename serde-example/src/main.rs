use serde::{Serialize, Deserialize};
use serde_json;
use postcard;

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    t: Example2,
    x: u32,
    y: i32,
    z: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Example2 {
    x: u32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    let a = Example { t: Example2 { x: 55, y: -55 }, x: 10, y: 20, z: String::from("Hello!") };
    println!("{:?}", a);
    let a_json = serde_json::to_string(&a).unwrap();
    println!("{}", a_json);
    let a_recovered = serde_json::from_str::<Example>(&a_json).unwrap();
    println!("{:?}", a_recovered);

    let b = Example { t: Example2 { x: 55, y: -55 }, x: 10, y: 20, z: String::from("Hello!") };
    println!("{:?}", b);
    let b_postcard = postcard::to_stdvec(&b).unwrap();
    println!("{:?}", b_postcard);
    let b_recovered = postcard::from_bytes::<Example>(&b_postcard).unwrap();
    println!("{:?}", b_recovered);

    let mut buf = [0u8; 128];
    let b_slice: &[u8] = postcard::to_slice(&b, &mut buf).unwrap();
    println!("{:?}", b_slice);
}

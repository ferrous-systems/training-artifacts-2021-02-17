mod workload;

mod folder_demo;
mod mod_demo;

use folder_demo::{
    folder_root, // func
    folder_contents::folder_contents_fn, // func
};

use mod_demo::{
    mod_demo_root,
    mod_contents::mod_contents_fn,
    ModDemo,
};

fn main_1() {
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

    let a = 42; // binds 42 to 'a'
    let _ = 42; // binds 42 to '_'
    let _a = 42;
}

fn foo(num: i64) -> bool {

    let _ = match num {
        // odd number
        x if x % 2 == 1 => {
            println!("odd");
            10
        }
        x if x == 100 => {
            println!("hundred");
            100
        }
        x => {
            println!("Default is: {}", x);
            1000
        }
    }; // destructor of _ runs HERE


    true // destructor of _y runs HERE
}

fn main() {
    workload::work();

    // let md = ModDemo {
    //     a: 10,
    //     b: String::from("hi"),
    //     c: 42.0f32
    // };

    let mut md = ModDemo::new(
        10,
        String::from("hi"),
        42.0f32
    );
    println!("{}", md.a);
    // println!("{}", md.b);
    dbg!(md.add());
    dbg!(md.see_string());
    md.set_c(30f32);
    dbg!(md.see_string());
    md.a = 20;

}

mod sub {
    // this is a module INSIDE of the `scratch` crate.
    //
    // from somewhere else:
    // use scratch::sub::*;
    mod subsub {
        mod subsubsub1 {
            // use scratch::sub::subsub::subsubsub2;
        }
        mod subsubsub2 {
            // use scratch::sub::subsub::subsubsub2::*;
        }
    }
}

// use scratch::file_a::sub as sub_a;
// use scratch::file_b::sub as sub_b;

// using namespace std;
// use std::*;

// std::whatever
// whatever

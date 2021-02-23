// Pull in eval and parse from the lib.rs/crate.

// use calc::{
//     // Things from our library
//     eval::eval,
//     Expr,
// };
use calc::prelude::*;

fn main() {
    println!("Input: '{}'", "3 sqr");
    // parse
    let res_parse = "3 sqr".parse::<Expr>();
    let parsed = res_parse.unwrap();

    // eval
    let evald = eval(&parsed).unwrap();

    // print
    println!("Output: '{}'", evald);
}

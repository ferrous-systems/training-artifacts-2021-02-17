use calc::{
    parse::parse,
    eval::eval,
};

#[test]
fn round_trip_add() {
    let text = "3 2 +";
    let expected = 5i64;
    let parsed = parse(text).unwrap();
    let evald = eval(&parsed).unwrap();
    assert_eq!(expected, evald);
}

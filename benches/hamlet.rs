#![feature(test)]

#[macro_use]
extern crate hamlet;
extern crate test;

use test::{ Bencher, black_box };

fn run(mut s: String) -> String {
    use std::fmt::Write;
    let events = [
        hamlet::Token::start_tag("div", attrs!()),
        hamlet::Token::text("Hello world!"),
        hamlet::Token::end_tag("div"),
    ];
    for event in &events {
        write!(s, "{}", event).unwrap();
    }
    s
}

#[test]
fn test() {
    assert_eq!(
        run(String::new()),
        "<div>Hello world!</div>");
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| run(black_box(String::new())))
}

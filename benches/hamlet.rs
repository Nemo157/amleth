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

fn prepare() -> [hamlet::Token<'static>; 3] {
    [
        hamlet::Token::start_tag("div", attrs!()),
        hamlet::Token::text("Hello world!"),
        hamlet::Token::end_tag("div"),
    ]
}

fn run_prepared(events: &[hamlet::Token<'static>; 3], mut s: String) -> String {
    use std::fmt::Write;
    for event in events {
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

#[test]
fn test_prepared() {
    let events = prepare();
    assert_eq!(
        run_prepared(&events, String::new()),
        "<div>Hello world!</div>");
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| run(black_box(String::new())))
}

#[bench]
fn bench_prepared(b: &mut Bencher) {
    let events = prepare();
    b.iter(|| run_prepared(&events, black_box(String::new())))
}

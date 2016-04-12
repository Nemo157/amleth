#![feature(test)]
#![feature(fnbox)]

#[macro_use]
extern crate amleth;
extern crate test;

use test::{ Bencher, black_box };

fn run(mut s: String) -> String {
    use std::fmt::Write;
    let events = html!({
        div {
            %("Hello world!")
        }
    });
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

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| run(black_box(String::new())))
}

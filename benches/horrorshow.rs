#![feature(test)]

#[macro_use]
extern crate horrorshow;
extern crate test;

use horrorshow::prelude::*;
use test::{ Bencher, black_box };

fn run(mut s: String) -> String {
    let tmp = html! {
        div {
            : "Hello world!"
        }
    };
    tmp.write_to_string(&mut s).unwrap();
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

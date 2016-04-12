#![feature(test)]
#![feature(plugin)]

#![plugin(maud_macros)]

extern crate maud;
extern crate test;

use test::{ Bencher, black_box };

fn run(mut s: String) -> String {
    html!(s, {
        div {
            ^"Hello world!"
        }
    }).unwrap();
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

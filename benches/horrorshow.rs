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

fn prepare() -> Box<Render> {
    box_html! {
        div {
            : "Hello world!"
        }
    }
}

fn run_prepared(tmp: &Render, mut s: String) -> String {
    tmp.write_to_string(&mut s).unwrap();
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
    let tmp = prepare();
    assert_eq!(
        run_prepared(&*tmp, String::new()),
        "<div>Hello world!</div>");
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| run(black_box(String::new())))
}

#[bench]
fn bench_prepared(b: &mut Bencher) {
    let tmp = prepare();
    b.iter(|| run_prepared(&*tmp, black_box(String::new())))
}

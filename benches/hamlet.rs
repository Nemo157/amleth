#![feature(test)]

#[macro_use]
extern crate hamlet;
extern crate test;

use hamlet::Token;
use test::{ Bencher, black_box };

fn run(mut s: String) -> String {
    use std::fmt::Write;
    let events = [
        Token::start_tag("div", attrs!()),
        Token::text("Hello world!"),
        Token::end_tag("div"),
    ];
    for event in &events {
        write!(s, "{}", event).unwrap();
    }
    s
}

fn run_no_fmt(mut s: String) -> String {
    struct T<'a, 'b: 'a>(&'a Token<'b>);

    impl<'a, 'b> T<'a, 'b> {
        fn write_to(&self, s: &mut String) {
            match *self.0 {
                Token::StartTag { ref name, ref attrs, self_closing } => {
                    s.push('<');
                    s.push_str(name);
                    for _ in attrs.iter() {
                        unimplemented!();
                    }
                    if self_closing {
                        s.push_str(" />")
                    } else {
                        s.push('>')
                    }
                }
                Token::EndTag { ref name } => {
                    s.push_str("</");
                    s.push_str(name);
                    s.push('>');
                }
                Token::Text(ref text) => {
                    // No escaping...
                    s.push_str(text);
                }
                Token::RawText(_) => unimplemented!(),
                Token::Comment(_) => unimplemented!(),
                Token::DOCTYPE => unimplemented!(),
            }
        }
    }

    let events = [
        Token::start_tag("div", attrs!()),
        Token::text("Hello world!"),
        Token::end_tag("div"),
    ];
    for event in &events {
        T(event).write_to(&mut s);
    }
    s
}

fn prepare() -> [Token<'static>; 3] {
    [
        Token::start_tag("div", attrs!()),
        Token::text("Hello world!"),
        Token::end_tag("div"),
    ]
}

fn run_prepared(events: &[Token<'static>; 3], mut s: String) -> String {
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
fn test_no_fmt() {
    assert_eq!(
        run_no_fmt(String::new()),
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
fn bench_no_fmt(b: &mut Bencher) {
    b.iter(|| run_no_fmt(black_box(String::new())))
}

#[bench]
fn bench_prepared(b: &mut Bencher) {
    let events = prepare();
    b.iter(|| run_prepared(&events, black_box(String::new())))
}

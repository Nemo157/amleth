#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(fnbox)]

#[macro_use]
extern crate hamlet;

macro_rules! html {
    ({ $($input:tt)* }) => {{
        html!(@element start ($($input)*) [@end] {
            first => first
        })
    }};

    (@finish { $current:expr }) => {{
        use ::hamlet::Token;
        use ::std::boxed::FnBox;
        struct CreateNext(Box<FnBox() -> Option<(Token<'static>, CreateNext)>>);
        struct Html { current: Option<CreateNext> }
        impl Iterator for Html {
            type Item = Token<'static>;
            fn next(&mut self) -> Option<Token<'static>> {
                if let Some(current) = self.current.take() {
                    if let Some((ret, next)) = current.0() {
                        self.current = Some(next);
                        Some(ret)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
        Html { current: Some($current) }
    }};

    (@end { $($current:tt)* }) => {{
        html!(@finish {
            match CreateNext(Box::new(move || None)) {
                $($current)*
            }
        })
    }};

    (@done [$($ret:tt)+] () { $($current:tt)* }) => {{
        html!($($ret)* { $($current)* })
    }};

    (@done [$($ret:tt)*] ($($tail:tt)+) { $($current:tt)* }) => {{
        html!(@element start ($($tail)*) [$($ret)*] { $($current)* })
    }};

    (@element end [$($ret:tt)*] ($tag:ident $($tail:tt)*) { $($current:tt)* }) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            next => match CreateNext(Box::new(move || Some((Token::end_tag(stringify!($tag)), next)))) {
                $($current)*
            }
        })
    }};

    (@element inner [$($ret:tt)*]
        ($tag:ident { $($inner:tt)* } $($tail:tt)*) {
            $($current:tt)*
        }
    ) => {{
        html!(@element start ($($inner)*) [@element end [$($ret)*] ($tag $($tail)*)] {
            $($current)*
        })
    }};

    (@element start
        (%($e:expr) $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            next => match CreateNext(Box::new(move || Some((Token::text($e), next)))) {
                $($current)*
            }
        })
    }};

    (@if end ($e:expr) { $($then:tt)* } { $($tail:tt)* } [$($ret:tt)*] { $($otherwise:tt)* }) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            next => match CreateNext(Box::new(move || {
                Some((Token::raw_text("/"), match if $e { match next { $($then)* } } else { match next { $($otherwise)* } } {
                    branch => branch
                }))
            })) {
                t => t
            }
        })
    }};

    (@if else ($e:expr) { $($otherwise:tt)* } { $($current:tt)* } { $($tail:tt)* } [$($ret:tt)*] { $($then:tt)* }) => {{
        html!(@element start ($($otherwise)*) [@if end ($e) { $($then)* } { $($tail)* } [$($ret)*]] {
            $($current)*
        })
    }};

    (@element start
        (if $e:expr => { $($then:tt)* } else { $($otherwise:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html!(@element start ($($then)*) [@if else ($e) { $($otherwise)* } { $($current)* } { $($tail)* } [$($ret)*]] {
            $($current)*
        })
    }};

    (@element start
        ($tag:tt { $($inner:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html!(@element inner [$($ret)*] ($tag { $($inner)* } $($tail)*) {
            next => match CreateNext(Box::new(move || Some((Token::start_tag(stringify!($tag), attrs!()), next)))) {
                $($current)*
            }
        })
    }};
}

pub mod test {
    use hamlet::Token;
    pub fn it_works() {
        // trace_macros!(true);
        let mut you_are_cool = false;
        let html = html!({
            div {
                p {
                    %("Hello, world!")
                    if you_are_cool => {
                        small { %(" and you :wink:") }
                    } else {
                        small { %(" except you :squint:") }
                    }
                }
            }
        });
        let html2 = html!({
            div {
                p {
                    %("Hello, world!")
                    if you_are_cool => {
                        small { %(" and you :wink:") }
                    } else {
                        small { %(" except you :squint:") }
                    }
                }
            }
        });
        you_are_cool = true;
        assert_eq!(html.collect::<Vec<_>>(), vec![
            Token::start_tag("div", attrs!()),
            Token::start_tag("p", attrs!()),
            Token::text("Hello, world!"),
            Token::start_tag("small", attrs!()),
            Token::text(" and you :wink:"),
            Token::end_tag("small"),
            Token::end_tag("p"),
            Token::end_tag("div"),
        ]);
        you_are_cool = false;
        assert_eq!(html2.collect::<Vec<_>>(), vec![
            Token::start_tag("div", attrs!()),
            Token::start_tag("p", attrs!()),
            Token::text("Hello, world!"),
            Token::start_tag("small", attrs!()),
            Token::text(" except you :squint:"),
            Token::end_tag("small"),
            Token::end_tag("p"),
            Token::end_tag("div"),
        ]);
    }
}

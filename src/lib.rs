#![feature(trace_macros)]
#![feature(log_syntax)]

#[macro_use]
extern crate hamlet;

macro_rules! html {
    ({ $($input:tt)* }) => {{
        html!(@element start ($($input)*) [] { })
    }};

    (@done []
        () {
            $($value:expr,)*
        }
    ) => {{
        use ::hamlet::Token;
        struct Html {
            values: Vec<Token<'static>>,
        }
        impl Iterator for Html {
            type Item = Token<'static>;
            fn next(&mut self) -> Option<Token<'static>> {
                self.values.pop()
            }
        }
        Html {
            values: vec![$($value,)*],
        }
    }};

    (@done [$($ret:tt)+]
        () {
            $($value:expr,)*
        }
    ) => {{
        html!($($ret)* { $($value,)* })
    }};

    (@done [$($ret:tt)*] ($($tail:tt)+) { $($value:expr,)* }) => {{
        html!(@element start ($($tail)*) [$($ret)*] { $($value,)* })
    }};

    (@element end [$($ret:tt)*] ($tag:ident $($tail:tt)*) { $($value:expr,)* }) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            Token::end_tag(stringify!($tag)),
            $($value,)*
        })
    }};

    (@element inner [$($ret:tt)*]
        ($tag:ident { $($inner:tt)* } $($tail:tt)*) {
            $($value:expr,)*
        }
    ) => {{
        log_syntax!(inner $($inner)*);
        html!(@element start ($($inner)*) [@element end [$($ret)*] ($tag $($tail)*)] {
            $($value,)*
        })
    }};

    (@element start
        (%($e:expr) $($tail:tt)*) [$($ret:tt)*] {
            $($value:expr,)*
        }
    ) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            Token::text($e),
            $($value,)*
        })
    }};

    (@element start
        ($tag:tt { $($inner:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($value:expr,)*
        }
    ) => {{
        html!(@element inner [$($ret)*] ($tag { $($inner)* } $($tail)*) {
            Token::start_tag(stringify!($tag), attrs!()),
            $($value,)*
        })
    }};
}

#[cfg(test)]
mod test {
    use hamlet::Token;
    #[test]
    fn it_works() {
        trace_macros!(true);
        let html = html!({
            div {
                p {
                    %("Hello, world!")
                    small { %(" and you :wink:") }
                }
            }
        });
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
    }
}

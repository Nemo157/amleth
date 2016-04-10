#![feature(trace_macros)]
#![feature(log_syntax)]

#[macro_use]
extern crate hamlet;

macro_rules! html {
    ({ $($input:tt)* }) => {{
        struct Html { values: Vec<Token<'static>> }
        html!(@element start Html ($($input)*) [] { })
    }};

    (@done $id:ident []
        () {
            $($value:expr,)*
        }
    ) => {{
        use ::hamlet::Token;
        impl Iterator for $id {
            type Item = Token<'static>;
            fn next(&mut self) -> Option<Token<'static>> {
                self.values.pop()
            }
        }
        $id { values: vec![$($value,)*] }
    }};

    (@done $id:ident [$($ret:tt)+]
        () {
            $($value:expr,)*
        }
    ) => {{
        html!($($ret)* { $($value,)* })
    }};

    (@done $id:ident [$($ret:tt)*] ($($tail:tt)+) { $($value:expr,)* }) => {{
        html!(@element start $id ($($tail)*) [$($ret)*] { $($value,)* })
    }};

    (@element end $id:ident [$($ret:tt)*] ($tag:ident $($tail:tt)*) { $($value:expr,)* }) => {{
        html!(@done $id [$($ret)*] ($($tail)*) {
            Token::end_tag(stringify!($tag)),
            $($value,)*
        })
    }};

    (@element inner $id:ident [$($ret:tt)*]
        ($tag:ident { $($inner:tt)* } $($tail:tt)*) {
            $($value:expr,)*
        }
    ) => {{
        log_syntax!(inner $($inner)*);
        html!(@element start $id ($($inner)*) [@element end $id [$($ret)*] ($tag $($tail)*)] {
            $($value,)*
        })
    }};

    (@element start $id:ident
        (%($e:expr) $($tail:tt)*) [$($ret:tt)*] {
            $($value:expr,)*
        }
    ) => {{
        html!(@done $id [$($ret)*] ($($tail)*) {
            Token::text($e),
            $($value,)*
        })
    }};

    (@element start $id:ident
        (if $e:expr => { $($then:tt)* } else { $($otherwise:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($value:expr,)*
        }
    ) => {{
        log_syntax!("boom!");
        if $e {
            html!(@element start $id ($($then)*) [@done $id [$($ret)*] ($($tail)*)] {
                $($value,)*
            })
        } else {
            html!(@element start $id ($($otherwise)*) [@done $id [$($ret)*] ($($tail)*)] {
                $($value,)*
            })
        }
    }};

    (@element start $id:ident
        ($tag:tt { $($inner:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($value:expr,)*
        }
    ) => {{
        html!(@element inner $id [$($ret)*] ($tag { $($inner)* } $($tail)*) {
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
        let you_are_cool = false;
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
        you_are_cool = true;
        assert_eq!(html, vec![
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

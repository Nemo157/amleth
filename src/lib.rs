#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(fnbox)]

#[cfg_attr(test, macro_use)]
pub extern crate hamlet;

use hamlet::Token;

pub struct CreateNext(pub Box<std::boxed::FnBox() -> Option<(Option<Token<'static>>, CreateNext)>>);

pub struct Html {
    pub current: Option<CreateNext>
}

impl Iterator for Html {
    type Item = Token<'static>;
    fn next(&mut self) -> Option<Token<'static>> {
        if let Some(current) = self.current.take() {
            if let Some((ret, next)) = current.0() {
                self.current = Some(next);
                ret.or_else(|| self.next())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! html {
    ({ $($input:tt)* }) => {{
        html!(@element start ($($input)*) [@end] {
            first => first
        })
    }};

    (@finish { $current:expr }) => {{
        $crate::Html { current: Some($current) }
    }};

    (@end { $($current:tt)* }) => {{
        html!(@finish {
            match $crate::CreateNext(Box::new(move || None)) {
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
            next => match $crate::CreateNext(Box::new(move || Some((Some($crate::hamlet::Token::end_tag(stringify!($tag))), next)))) {
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
            next => match $crate::CreateNext(Box::new(move || Some((Some($crate::hamlet::Token::text($e)), next)))) {
                $($current)*
            }
        })
    }};

    (@if end ($e:expr) { $($then:tt)* } { $($tail:tt)* } [$($ret:tt)*] { $($otherwise:tt)* }) => {{
        html!(@done [$($ret)*] ($($tail)*) {
            next => match $crate::CreateNext(Box::new(move || {
                Some((None, match if $e { match next { $($then)* } } else { match next { $($otherwise)* } } {
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
            next => match $crate::CreateNext(Box::new(move || Some((Some($crate::hamlet::Token::start_tag(stringify!($tag), $crate::hamlet::attr::AttributeList::empty())), next)))) {
                $($current)*
            }
        })
    }};
}

pub trait Recurse {
    type Next: Recurse;
    fn recurse<F: FnMut(Token<'static>)>(self, f: F);
}

pub struct CreateNextUnboxed<N: Recurse, F: FnOnce() -> Option<(Option<Token<'static>>, N)>>(F);

impl<N: Recurse, F: FnOnce() -> Option<(Option<Token<'static>>, N)>> Recurse for CreateNextUnboxed<N, F> {
    type Next = N;
    fn recurse<F2: FnMut(Token<'static>)>(self, mut f: F2) {
        if let Some((ret, next)) = (self.0)() {
            if let Some(ret) = ret {
                f(ret);
            }
            next.recurse(f);
        }
    }
}

pub struct HtmlUnboxed<N: Recurse> {
    root: N,
}

impl<N: Recurse> HtmlUnboxed<N> {
    pub fn iterate<F: FnMut(Token<'static>)>(self, f: F) {
        self.root.recurse(f);
    }
}

#[macro_export]
macro_rules! html_unboxed {
    ({ $($input:tt)* }) => {{
        html_unboxed!(@element start ($($input)*) [@end] {
            first => first
        })
    }};

    (@finish { $current:expr }) => {{
        $crate::HtmlUnboxed { root: $current }
    }};

    (@end { $($current:tt)* }) => {{
        html_unboxed!(@finish {
            match $crate::CreateNextUnboxed(move || None) {
                $($current)*
            }
        })
    }};

    (@done [$($ret:tt)+] () { $($current:tt)* }) => {{
        html_unboxed!($($ret)* { $($current)* })
    }};

    (@done [$($ret:tt)*] ($($tail:tt)+) { $($current:tt)* }) => {{
        html_unboxed!(@element start ($($tail)*) [$($ret)*] { $($current)* })
    }};

    (@element end [$($ret:tt)*] ($tag:ident $($tail:tt)*) { $($current:tt)* }) => {{
        html_unboxed!(@done [$($ret)*] ($($tail)*) {
            next => match $crate::CreateNextUnboxed(move || Some((Some(Token::end_tag(stringify!($tag))), next))) {
                $($current)*
            }
        })
    }};

    (@element inner [$($ret:tt)*]
        ($tag:ident { $($inner:tt)* } $($tail:tt)*) {
            $($current:tt)*
        }
    ) => {{
        html_unboxed!(@element start ($($inner)*) [@element end [$($ret)*] ($tag $($tail)*)] {
            $($current)*
        })
    }};

    (@element start
        (%($e:expr) $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html_unboxed!(@done [$($ret)*] ($($tail)*) {
            next => match $crate::CreateNextUnboxed(move || Some((Some(Token::text($e)), next))) {
                $($current)*
            }
        })
    }};

    (@if end ($e:expr) { $($then:tt)* } { $($tail:tt)* } [$($ret:tt)*] { $($otherwise:tt)* }) => {{
        html_unboxed!(@done [$($ret)*] ($($tail)*) {
            next => match $crate::CreateNextUnboxed(move || {
                Some((None, match if $e { match next { $($then)* } } else { match next { $($otherwise)* } } {
                    branch => branch
                }))
            }) {
                t => t
            }
        })
    }};

    (@if else ($e:expr) { $($otherwise:tt)* } { $($current:tt)* } { $($tail:tt)* } [$($ret:tt)*] { $($then:tt)* }) => {{
        html_unboxed!(@element start ($($otherwise)*) [@if end ($e) { $($then)* } { $($tail)* } [$($ret)*]] {
            $($current)*
        })
    }};

    (@element start
        (if $e:expr => { $($then:tt)* } else { $($otherwise:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html_unboxed!(@element start ($($then)*) [@if else ($e) { $($otherwise)* } { $($current)* } { $($tail)* } [$($ret)*]] {
            $($current)*
        })
    }};

    (@element start
        ($tag:tt { $($inner:tt)* } $($tail:tt)*) [$($ret:tt)*] {
            $($current:tt)*
        }
    ) => {{
        html_unboxed!(@element inner [$($ret)*] ($tag { $($inner)* } $($tail)*) {
            next => match $crate::CreateNextUnboxed(move || Some((Some(Token::start_tag(stringify!($tag), $crate::hamlet::attr::AttributeList::empty())), next))) {
                $($current)*
            }
        })
    }};
}


#[cfg(test)]
pub mod test {
    use hamlet::Token;

    #[test]
    pub fn it_works() {
        // trace_macros!(true);
        let mut you_are_cool = true;
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
        you_are_cool = false;
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

    #[test]
    pub fn it_works_unboxed() {
        // trace_macros!(true);
        let mut you_are_cool = true;
        let html = html_unboxed!({
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
        you_are_cool = false;
        let html2 = html_unboxed!({
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

pub mod alias;
mod build_html;

use std::borrow::Cow;

use html::inline_text::Span;

#[macro_export]
macro_rules! quote {
    ($($tt:tt)*) => {
        $crate::html!(i "\"" $($tt)* "\"").build()
    };
}

#[inline]
pub fn typo(typo: impl Into<Cow<'static, str>>, untypo: impl Into<Cow<'static, str>>) -> [Span; 2] {
    [
        Span::builder().class("untypo").text(untypo).build(),
        Span::builder().class("typo").text(typo).build(),
    ]
}

use crate::entry::{Language, Qualifier};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attachment<'a> {
    Alternative(&'a str),
    Etymology(&'a [&'a str]),
    Language(Language),
    Gloss(&'a str),
    Qualifier(Qualifier),
    // Reference(Reference), NOT USED it seems only in original Maugnayin
}

#[inline]
#[must_use]
pub const fn alternative<'a>(alternative: &'a [&'a str; 1]) -> Attachment<'a> {
    Attachment::Alternative(alternative[0])
}

#[inline]
#[must_use]
pub const fn etymology<'a>(etymology: &'a [&'a str]) -> Attachment<'a> {
    Attachment::Etymology(etymology)
}

#[inline]
#[must_use]
pub const fn language<'a>(language: &[Language; 1]) -> Attachment<'a> {
    Attachment::Language(language[0])
}

#[inline]
#[must_use]
pub const fn gloss<'a>(gloss: &'a [&'a str; 1]) -> Attachment<'a> {
    Attachment::Gloss(gloss[0])
}

#[inline]
#[must_use]
pub const fn qualifier<'a>(qualifier: &[Qualifier; 1]) -> Attachment<'a> {
    Attachment::Qualifier(qualifier[0])
}

pub mod alias {
    pub use super::{alternative as a, etymology as e, gloss as g, language as l, qualifier as q};
}

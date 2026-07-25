use std::{fmt, num::{NonZeroU8, NonZeroU16, NonZeroUsize}};

use crate::Text;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Reference<'a> {
    pub abbr: Text<'a>,
    pub title: Text<'a>,
    pub subtitle: Option<Text<'a>>,
    pub publisher: Option<Text<'a>>,
    /// Publication location.
    pub publication: Option<Text<'a>>,
    /// Published date.
    pub published: Option<NonZeroUsize>,
}

#[inline]
const fn txt(text: &'static str, lang: &'static str) -> Text<'static> {
    Text::new(text, lang).untranslatable()
}

pub const SWP: Reference = Reference {
    abbr: txt("SWP", "fil-PH"),
    title: txt("Surian ng Wikang Pambansa", "fil-PH"),
    subtitle: Some(txt("An English-Taagalog Dictionary", "en")),
    publisher: Some(txt("Bureau of Printing", "en")),
    publication: Some(Text::new("Maynila", "fil")),
    published: NonZeroUsize::new(1960),
};

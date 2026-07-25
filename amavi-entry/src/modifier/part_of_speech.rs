#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartOfSpeech {
    Adjective,
    Noun,
    Verb,
}

impl PartOfSpeech {
    #[inline]
    #[must_use]
    pub const fn abbr(self) -> &'static str {
        match self {
            Self::Adjective => "adj.",
            Self::Noun => "n.",
            Self::Verb => "v.",
        }
    }

    #[inline]
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Adjective => "Adjective",
            Self::Noun => "Noun",
            Self::Verb => "Verb",
        }
    }
}

pub mod alias {
    #[allow(unused_imports)]
    pub use super::PartOfSpeech::{Adjective as adj, Noun as n, Verb as v};
}

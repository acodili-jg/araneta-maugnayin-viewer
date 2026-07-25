use crate::Modifier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Term<'a>(&'a [Fragment<'a>]);

impl<'a> Term<'a> {
    #[inline]
    #[must_use]
    pub const fn new(fragments: &'a [Fragment<'a>]) -> Self {
        Self(fragments)
    }

    #[inline]
    #[must_use]
    pub const fn fragments(&self) -> &'a [Fragment<'a>] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fragment<'a> {
    language: Option<super::Language>,
    text: &'a str,
    modifiers: &'a [Modifier<'a>],
}

impl<'a> Fragment<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        language: Option<super::Language>,
        text: &'a str,
        modifiers: &'a [Modifier<'a>],
    ) -> Self {
        Self {
            language,
            text,
            modifiers,
        }
    }

    #[inline]
    #[must_use]
    pub const fn new_plain(text: &'a str) -> Self {
        Self::new(None, text, &[])
    }

    #[inline]
    #[must_use]
    pub const fn language(&self) -> Option<super::Language> {
        self.language
    }

    #[inline]
    #[must_use]
    pub const fn get_language(self) -> Option<super::Language> {
        self.language
    }

    #[inline]
    #[must_use]
    pub const fn text(&self) -> &'a str {
        self.text
    }

    #[inline]
    #[must_use]
    pub const fn get_text(self) -> &'a str {
        self.text
    }

    #[inline]
    #[must_use]
    pub const fn modifiers(&self) -> &'a [Modifier<'_>] {
        self.modifiers
    }

    #[inline]
    #[must_use]
    pub const fn get_modifiers(self) -> &'a [Modifier<'a>] {
        self.modifiers
    }
}

impl Fragment<'static> {
    pub const HYPHEN: Self = Self::new_plain("-");

    pub const SPACE: Self = Self::new_plain(" ");
}

use crate::entry_v2::Modifier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Term<'a>(&'a [Fragment<'a>]);

impl<'a> Term<'a> {
    #[inline]
    #[must_use] 
    pub const fn new(fragments: &'a [Fragment<'a>]) -> Self {
        Self(fragments)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fragment<'a> {
    language: Option<super::Language>,
    text: &'a str,
    modifiers: &'a [Modifier<'a>],
}

impl Fragment<'_> {
    #[inline]
    pub const fn language(&self) -> Option<super::Language> {
        self.language
    }

    #[inline]
    pub const fn text(&self) -> &str {
        self.text
    }

    #[inline]
    pub const fn modifiers(&self) -> &[Modifier<'_>] {
        self.modifiers
    }
}

impl<'a> Fragment<'a> {
    #[inline]
    pub const fn new(
        language: Option<super::Language>,
        text: &'a str,
        modifiers: &'a [Modifier<'a>],
    ) -> Self {
        Self { language, text, modifiers }
    }

    #[inline]
    pub const fn new_plain(text: &'a str) -> Self {
        Self::new(None, text, &[])
    }

    #[inline]
    pub const fn get_language(self) -> Option<super::Language> {
        self.language
    }

    #[inline]
    pub const fn get_text(self) -> &'a str {
        self.text
    }

    #[inline]
    pub const fn get_modifiers(self) -> &'a [Modifier<'a>] {
        self.modifiers
    }
}

impl Fragment<'static> {
    pub const HYPHEN: Self = Self::new_plain("-");

    pub const SPACE: Self = Self::new_plain(" ");
}

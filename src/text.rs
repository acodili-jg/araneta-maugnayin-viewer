#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Text<'a> {
    pub text: &'a str,
    pub lang: &'a str,
    pub translate: Option<bool>,
}

impl<'a> Text<'a> {
    #[inline]
    #[must_use]
    pub const fn new(text: &'a str, lang: &'a str) -> Self {
        Self {
            text,
            lang,
            translate: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn translate(mut self, translate: bool) -> Self {
        self.translate = Some(translate);
        self
    }

    #[inline]
    #[must_use]
    pub const fn untranslatable(self) -> Self {
        self.translate(false)
    }
}

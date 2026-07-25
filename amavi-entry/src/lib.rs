#![feature(const_index)]
#![feature(const_trait_impl)]

mod html;
pub mod language;
mod macros;
pub mod modifier;
pub mod page;
pub mod term;

pub use language::Language;
pub use modifier::Modifier;
pub use page::Page;
pub use term::Term;

#[derive(Clone, Copy, Debug)]
pub struct Entries<'a>(&'a [Page<'a>]);

impl<'a> Entries<'a> {
    #[inline]
    #[must_use]
    pub const fn new(pages: &'a [Page<'a>]) -> Self {
        Self(pages)
    }

    #[inline]
    #[must_use]
    pub const fn pages(&self) -> &'a [Page<'a>] {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Entry<'a>(pub Mapping<'a>, pub Mapping<'a>);

#[derive(Clone, Copy, Debug)]
pub struct Mapping<'a>(&'a [Term<'a>]);

impl<'a> Mapping<'a> {
    #[inline]
    #[must_use]
    pub const fn new(terms: &'a [Term<'a>]) -> Self {
        Self(terms)
    }

    #[inline]
    #[must_use]
    pub const fn terms(&self) -> &'a [Term<'a>] {
        self.0
    }
}

impl Entries<'_> {
    #[inline]
    #[must_use]
    pub const fn count(&self) -> usize {
        let mut count = 0;

        let mut idx = 0;
        while let Some(page) = self.0.get(idx) {
            count += page.children().len();
            idx += 1;
        }

        count
    }
}

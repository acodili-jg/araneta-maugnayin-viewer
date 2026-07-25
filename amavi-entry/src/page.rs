use super::Entry;

#[derive(Clone, Copy, Debug)]
pub struct Page<'a>(&'a [Entry<'a>]);

impl<'a> Page<'a> {
    #[inline]
    #[must_use]
    pub const fn new(entries: &'a [Entry<'a>]) -> Self {
        Self(entries)
    }

    #[inline]
    #[must_use]
    pub const fn children(&self) -> &'a [Entry<'a>] {
        self.0
    }
}

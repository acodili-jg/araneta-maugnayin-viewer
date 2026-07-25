use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Counter<L>
where
    L: Label,
{
    count: usize,
    label: PhantomData<L>,
}

impl<L> Counter<L>
where
    L: Label,
{
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::start(1)
    }

    #[inline]
    #[must_use]
    pub const fn start(count: usize) -> Self {
        Self {
            count,
            label: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn current(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn increment(&mut self) -> String {
        let string = L::format(self.count);
        self.count += 1;
        string
    }
}

impl<L> Default for Counter<L>
where
    L: Label,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub mod label {
    pub trait Label {
        fn format(count: usize) -> String;
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Simple;

    impl Label for Simple {
        #[inline]
        fn format(count: usize) -> String {
            count.to_string()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Page;

    impl Label for Page {
        #[inline]
        fn format(count: usize) -> String {
            format!("page-{count}")
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Paragraph;

    impl Label for Paragraph {
        #[inline]
        fn format(count: usize) -> String {
            format!("para-{count}")
        }
    }
}

pub use label::Label;

pub type Simple = Counter<label::Simple>;
pub type Page = Counter<label::Page>;
pub type Paragraph = Counter<label::Paragraph>;

use crate::Language;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Etymology<'a> {
    Affix(&'a [(Option<Language>, Option<&'a str>)]),
    Blend(&'a [(Option<Language>, Option<&'a str>)]),
    Borrow(&'a [(Option<Language>, Option<&'a str>)]),
    From(&'a [(Option<Language>, Option<&'a str>)]),
    Inherited(&'a [(Option<Language>, Option<&'a str>)]),
}

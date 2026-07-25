mod language;
mod modifier;
mod page;
mod term;

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
}

#[macro_export]
macro_rules! mapping_impl {
    (FINISH; $($terms:expr,)*) => {
        $crate::entry_v2::Mapping::new(&[$($terms,)*])
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
    ) => {
        $crate::mapping_impl!(FINISH; $($terms,)* $crate::entry_v2::Term::new(&[$($fragments,)*]),)
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        , $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
            START;
            $($terms,)* $crate::entry_v2::Term::new(&[$($fragments,)*]),;
            ;
            $($tokens)*
        )
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        $($languagecode:ident :)? $text:literal $($tokens:tt)*
    ) => {
        mapping_impl!(
            FRAGMENT;
            $($terms,)*;
            $($fragments,)*;
            $crate::language!($($languagecode)?);
            $text;
            ;
            $($tokens)*
        )
    };

    (
        FRAGMENT;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        $language:expr;
        $text:expr;
        $($modifiers:expr,)*;
        <$modifier:ident : $core:tt
            < $($languagecode_0:ident :)? $text_0:literal $(>
            < $($languagecode_n:ident :)? $text_n:literal
        )* >> $($tokens:tt)*
    ) => {
        mapping_impl!(
            FRAGMENT;
            $($terms,)*;
            $($fragments,)*;
            $language;
            $text;
            $($modifiers,)* $crate::modifier!($modifier : $core <$($languagecode_0 :)? $text_0> $(<$($languagecode_n :)? $text_n>)*),;
            $($tokens)*
        )
    };

    (
        FRAGMENT;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        $language:expr;
        $text:expr;
        $($modifiers:expr,)*;
        <$modifier:ident : $core:tt
            $(< $($languagecode_n:ident :)? $text_n:literal>)*
        > $($tokens:tt)*
    ) => {
        mapping_impl!(
            FRAGMENT;
            $($terms,)*;
            $($fragments,)*;
            $language;
            $text;
            $($modifiers,)* $crate::modifier!($modifier : $core $(<$($languagecode_n :)? $text_n>)*),;
            $($tokens)*
        )
    };

    (
        FRAGMENT;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        $language:expr; $text:expr;
        $($modifiers:expr,)*;
        $($tokens:tt)*
    ) => {
        mapping_impl!(
            START;
            $($terms,)*;
            $($fragments,)* $crate::entry_v2::term::Fragment::new($language, $text, &[$($modifiers,)*]),;
            $($tokens)*
        )
    };
}

#[macro_export]
macro_rules! mapping {
    ($($tokens:tt)*) => {
        $crate::mapping_impl!(START; ; ; $($tokens)*)
    };
}

#[macro_export]
macro_rules! entry {
    ([$($lhs:tt)*] => [$($rhs:tt)*]) => {
        $crate::entry_v2::Entry(
            $crate::mapping!($($lhs)*),
            $crate::mapping!($($rhs)*),
        )
    };
}

#[macro_export]
macro_rules! entry_page_impl {
    (
        { $($entries:expr,)* }
    ) => {
        $crate::entry_v2::Page::new(&[$($entries,)*])
    };

    (
        { $($entries:expr,)* }
        { $($captured:tt)* }
        $(,)?
    ) => {
        $crate::entry_page_impl!(
            { $($entries,)* $crate::entry!($($captured)*), }
        )
    };

    (
        { $($entries:expr,)* }
        { $($captured:tt)* }
        , $($tokens:tt)*
    ) => {
        $crate::entry_page_impl!(
            { $($entries,)* $crate::entry!($($captured)*), }
            {}
            $($tokens)*
        )
    };

    (
        { $($entries:expr,)* }
        { $($captured:tt)* }
        $token:tt $($tokens:tt)*
    ) => {
        $crate::entry_page_impl!(
            { $($entries,)* }
            { $($captured)* $token }
            $($tokens)*
        )
    };
}

#[macro_export]
macro_rules! entry_page {
    ($($tokens:tt)*) => {
        $crate::entry_page_impl!({} {} $($tokens)*)
    };
}

#[macro_export]
macro_rules! entries_v2 {
    ($([$($pages:tt)*]),* $(,)?) => {
        $crate::entry_v2::Entries::new(&[$($crate::entry_page!($($pages)*))*])
    };
}

const fn test() {
    // let _ = dbg!(mapping!("hello"));
    // let _ = dbg!(mapping!("hello"<alt:"goodbye">));
    let _ = entries_v2!([["one"] => ["isa"],["one"] => ["isa"]]);
}

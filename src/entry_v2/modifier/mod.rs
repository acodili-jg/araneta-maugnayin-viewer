use super::Language;

pub mod part_of_speech;
pub mod qualifier;

pub use part_of_speech::PartOfSpeech;
pub use qualifier::Qualifier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Modifier<'a> {
    Alternative(&'a str),
    Etymology(Etymology<'a>),
    Gloss(&'a str),
    PartOfSpeech(PartOfSpeech),
    Qualifier(Qualifier),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Etymology<'a> {
    Affix(&'a [(Option<Language>, &'a str)]),
}

#[macro_export]
macro_rules! modifier {
    (alt: $alternative_display:expr) => {
        $crate::entry_v2::Modifier::Alternative($alternative_display)
    };
    (ety: af $(< $($language:ident :)? $term:literal>)+) => {
        $crate::entry_v2::Modifier::Etymology(
            $crate::entry_v2::modifier::Etymology::Affix(&[
                $(($crate::language!($($language)?), $term), )+
            ])
        )
    };
    (t: $gloss:expr) => {
        $crate::entry_v2::Modifier::Gloss($gloss)
    };
    (pos: $part_of_speech:ident) => {
        $crate::entry_v2::Modifier::PartOfSpeech($crate::entry_v2::modifier::part_of_speech::alias::$part_of_speech)
    };
    (q: $qualifier:ident) => {
        $crate::entry_v2::Modifier::Qualifier($crate::entry_v2::modifier::qualifier::alias::$qualifier)
    };
}

fn test() {
    dbg!(modifier!(alt:"Test"));
    dbg!(modifier!(ety:af<"Test">));
    dbg!(modifier!(ety:af<MatTag:"Test">));
    dbg!(modifier!(t:"gloss"));
    dbg!(modifier!(pos:n));
    dbg!(modifier!(q:r#gen));
}

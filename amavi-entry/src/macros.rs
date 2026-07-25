#[macro_export]
macro_rules! language {
    () => {
        None
    };
    ($language:ident) => {
        Some($crate::language::alias::$language)
    };

    ($($language:ident)? : -) => {
        ($crate::language!($($language)?), None)
    };
    ($($language:ident)? : $text:expr) => {
        ($crate::language!($($language)?), Some($text))
    };
}

#[macro_export]
macro_rules! plain_term {
    (- $(,)?) => {
        (None, None)
    };
    ($term:expr $(,)?) => {
        (None, Some($term))
    };
    ($language:ident, - $(,)?) => {
        ($crate::language!($language), None)
    };
    ($language:ident, $text:expr $(,)?) => {
        ($crate::language!($language), Some($text))
    };
}

#[macro_export]
macro_rules! modifier {
    (alt: $alternative_display:expr) => {
        $crate::Modifier::Alternative($alternative_display)
    };
    (lalt: $alternative_display:expr) => {
        $crate::Modifier::AltLeft($alternative_display)
    };
    (ety: af $(< $language:tt $(: $term:tt)?>)+) => {
        $crate::Modifier::Etymology(
            $crate::modifier::Etymology::Affix(&[
                $($crate::plain_term!($language $(, $term)?), )+
            ])
        )
    };
    (ety: blend $(< $language:tt $(: $term:tt)?>)+) => {
        $crate::Modifier::Etymology(
            $crate::modifier::Etymology::Blend(&[
                $($crate::plain_term!($language $(, $term)?), )+
            ])
        )
    };
    (ety: bor $(< $language:tt $(: $term:tt)?>)+) => {
        $crate::Modifier::Etymology(
            $crate::modifier::Etymology::Borrow(&[
                $($crate::plain_term!($language $(, $term)?), )+
            ])
        )
    };
    (ety: from $(< $language:tt $(: $term:tt)?>)+) => {
        $crate::Modifier::Etymology(
            $crate::modifier::Etymology::From(&[
                $($crate::plain_term!($language $(, $term)?), )+
            ])
        )
    };
    (t: $gloss:expr) => {
        $crate::Modifier::Gloss($gloss)
    };
    (pos: $part_of_speech:ident) => {
        $crate::Modifier::PartOfSpeech($crate::modifier::part_of_speech::alias::$part_of_speech)
    };
    (q: $qualifier:ident) => {
        $crate::Modifier::Qualifier($crate::modifier::qualifier::alias::$qualifier)
    };
}

#[macro_export]
macro_rules! mapping_impl {
    (FINISH; $($terms:expr,)*) => {
        $crate::Mapping::new(&[$($terms,)*])
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
    ) => {
        $crate::mapping_impl!(FINISH; $($terms,)* $crate::Term::new(&[$($fragments,)*]),)
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        , $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
            START;
            $($terms,)* $crate::Term::new(&[$($fragments,)*]),;
            ;
            $($tokens)*
        )
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        - $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
            START;
            $($terms,)*;
            $($fragments,)* $crate::term::Fragment::HYPHEN,;
            $($tokens)*
        )
    };

    (
        START;
        $($terms:expr,)*;
        $($fragments:expr,)*;
        $($languagecode:ident :)? $text:literal $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
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
            < $languagecode_0:tt $(: $text_0:tt)? $(>
            < $languagecode_n:tt $(: $text_n:tt)?
        )* >> $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
            FRAGMENT;
            $($terms,)*;
            $($fragments,)*;
            $language;
            $text;
            $($modifiers,)* $crate::modifier!($modifier : $core <$languagecode_0 $(: $text_0)?> $(<$languagecode_n $(: $text_n)?>)*),;
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
            $(< $languagecode_n:tt $(: $text_n:tt)?>)*
        > $($tokens:tt)*
    ) => {
        $crate::mapping_impl!(
            FRAGMENT;
            $($terms,)*;
            $($fragments,)*;
            $language;
            $text;
            $($modifiers,)* $crate::modifier!($modifier : $core $(<$languagecode_n $(: $text_n)?>)*),;
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
        $crate::mapping_impl!(
            START;
            $($terms,)*;
            $($fragments,)* $crate::term::Fragment::new($language, $text, &[$($modifiers,)*]),;
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
        $crate::Entry(
            $crate::mapping!($($lhs)*),
            $crate::mapping!($($rhs)*),
        )
    };
}

#[macro_export]
macro_rules! page_impl {
    (
        { $($entries:expr,)* }
    ) => {
        $crate::Page::new(&[$($entries,)*])
    };

    (
        { $($entries:expr,)* }
        { $($captured:tt)* }
        $(,)?
    ) => {
        $crate::page_impl!(
            { $($entries,)* $crate::entry!($($captured)*), }
        )
    };

    (
        { $($entries:expr,)* }
        { $($captured:tt)* }
        , $($tokens:tt)*
    ) => {
        $crate::page_impl!(
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
        $crate::page_impl!(
            { $($entries,)* }
            { $($captured)* $token }
            $($tokens)*
        )
    };
}

#[macro_export]
macro_rules! page {
    ($($tokens:tt)*) => {
        $crate::page_impl!({} {} $($tokens)*)
    };
}

#[macro_export]
macro_rules! entries {
    ($([$($pages:tt)*]),* $(,)?) => {
        $crate::Entries::new(&[$(
            $crate::page!($($pages)*),
        )*])
    };
}

#[macro_export]
macro_rules! definitions {
    ($($tt:tt)*) => {
        pub const ENTRIES: $crate::Entries<'_> = $crate::entries!($($tt)*);

        #[allow(unused)]
        pub const COUNT: usize = ENTRIES.count();
    };
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Language {
    Bisaya,
    Bikol,
    Hiligaynon,
    Iluko,
    Kapampangan,
    Maranaw,
    MatandangTagalog,
    Tagalog,
}

impl Language {
    #[inline]
    #[must_use]
    pub const fn abbr(self) -> &'static str {
        match self {
            Self::Bisaya => "Bis.",
            Self::Bikol => "Bkl.",
            Self::Hiligaynon => "Hlg.",
            Self::Iluko => "Ilk.",
            Self::Kapampangan => "Kpn.",
            Self::Maranaw => "Mar.",
            Self::MatandangTagalog => "Mat. Tag.",
            Self::Tagalog => "Tag.",
        }
    }

    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bisaya => "Bisaya",
            Self::Bikol => "Bikol",
            Self::Hiligaynon => "Hiligaynon",
            Self::Iluko => "Iluko",
            Self::Kapampangan => "Kapampangan",
            Self::Maranaw => "Maranaw",
            Self::MatandangTagalog => "Matandang Tagalog",
            Self::Tagalog => "Tagalog",
        }
    }
}

pub mod alias {
    #[allow(unused_imports)]
    pub use super::Language::{
        Bikol as Bkl,
        Bisaya as Bis,
        Hiligaynon as Hlg,
        Iluko as Ilk,
        Kapampangan as Kpn,
        Maranaw as Mar,
        MatandangTagalog as MatTag,
        Tagalog as Tag,
    };
}

#[macro_export]
macro_rules! language {
    () => {
        None
    };
    ($language:ident) => {
        Some($crate::entry_v2::language::alias::$language)
    };
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Qualifier {
    DoctorOfPhilosophy,
    Education,
    General,
    Geometry,
    Linguistics,
    Logarithm,
    Mechanics,
    Mathematics,
    Ordinary,
    Statistics,
    Technical,
}

impl Qualifier {
    #[inline]
    #[must_use]
    pub const fn abbr(self) -> &'static str {
        match self {
            Self::DoctorOfPhilosophy => "PhD.",
            Self::Education => "Ed.",
            Self::General => "gen.",
            Self::Geometry => "Geo.",
            Self::Linguistics => "Ling.",
            Self::Logarithm => "log.",
            Self::Mechanics => "Mech.",
            Self::Mathematics => "Math.",
            Self::Ordinary => "ord.",
            Self::Statistics => "Sta.",
            Self::Technical => "tech.",
        }
    }

    #[inline]
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::DoctorOfPhilosophy => "Doctor of Philosophy",
            Self::Education => "Education",
            Self::General => "General",
            Self::Geometry => "Geometry",
            Self::Linguistics => "Linguistics",
            Self::Logarithm => "Logarithm",
            Self::Mechanics => "Mechanics",
            Self::Mathematics => "Mathematics",
            Self::Ordinary => "Ordinary",
            Self::Statistics => "Statistics",
            Self::Technical => "Technical",
        }
    }
}

pub mod alias {
    #[allow(unused_imports)]
    pub use super::Qualifier::{
        DoctorOfPhilosophy as PhD, Education as Ed, General as r#gen, Geometry as Geo,
        Linguistics as Ling, Logarithm as log, Mathematics as Math, Mechanics as Mech,
        Ordinary as ord, Statistics as Sta, Technical as tech,
    };
}

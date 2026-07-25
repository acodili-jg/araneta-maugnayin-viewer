pub mod etymology;
pub mod part_of_speech;
pub mod qualifier;

pub use etymology::Etymology;
pub use part_of_speech::PartOfSpeech;
pub use qualifier::Qualifier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Modifier<'a> {
    Alternative(&'a str),
    AltLeft(&'a str),
    Etymology(Etymology<'a>),
    Gloss(&'a str),
    PartOfSpeech(PartOfSpeech),
    Qualifier(Qualifier),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Left<'a> {
    Alternative(&'a str),
}

impl<'a> TryFrom<Modifier<'a>> for Left<'a> {
    type Error = Modifier<'a>;

    #[inline]
    fn try_from(modifier: Modifier<'a>) -> Result<Self, Self::Error> {
        Ok(match modifier {
            Modifier::AltLeft(alternative_display) => Self::Alternative(alternative_display),
            modifier => return Err(modifier),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Right<'a> {
    Alternative(&'a str),
    Etymology(Etymology<'a>),
    Gloss(&'a str),
    PartOfSpeech(PartOfSpeech),
    Qualifier(Qualifier),
}

impl<'a> TryFrom<Modifier<'a>> for Right<'a> {
    type Error = Modifier<'a>;

    #[inline]
    fn try_from(modifier: Modifier<'a>) -> Result<Self, Self::Error> {
        Ok(match modifier {
            Modifier::Alternative(alternative_display) => Self::Alternative(alternative_display),
            Modifier::Etymology(etymology) => Self::Etymology(etymology),
            Modifier::Gloss(gloss) => Self::Gloss(gloss),
            Modifier::PartOfSpeech(part_of_speech) => Self::PartOfSpeech(part_of_speech),
            Modifier::Qualifier(qualifier) => Self::Qualifier(qualifier),
            modifier => return Err(modifier),
        })
    }
}

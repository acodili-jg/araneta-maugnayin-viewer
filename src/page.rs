use crate::{Text, counter};
use html::content::{Main, builders::MainBuilder};

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub id: &'static str,
    pub name: Text<'static>,
    pub main: Main,
    pub lang: Option<&'static str>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Builder {
    pages: Vec<Page>,
    paragraph: counter::Paragraph,
}

pub struct BuildContext<'a> {
    owner: &'a mut Builder,
    id: &'static str,
    name: Text<'static>,
    lang: Option<&'static str>,
    builder: MainBuilder,
}

pub struct UnpackedBuildContext<'a> {
    pub builder: &'a mut MainBuilder,
    pub paragraph: &'a mut counter::Paragraph,
}

impl Builder {
    #[inline]
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            pages: Vec::new(),
            paragraph: counter::Paragraph::new(),
        }
    }

    #[inline]
    #[must_use] 
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pages: Vec::with_capacity(capacity),
            paragraph: counter::Paragraph::new(),
        }
    }

    #[inline]
    #[must_use] 
    pub const fn pages(&self) -> &[Page] {
        self.pages.as_slice()
    }

    #[inline]
    #[must_use] 
    pub fn into_pages(self) -> Vec<Page> {
        self.pages
    }

    #[inline]
    pub fn start(
        &mut self,
        id: &'static str,
        name: Text<'static>,
        lang: Option<&'static str>,
    ) -> BuildContext<'_> {
        BuildContext {
            owner: self,
            id,
            name,
            lang,
            builder: Main::builder(),
        }
    }
}

impl BuildContext<'_> {
    #[inline]
    #[must_use] 
    pub const fn id(&self) -> &'static str {
        self.id
    }

    #[inline]
    #[must_use] 
    pub const fn name(&self) -> Text<'static> {
        self.name
    }

    #[inline]
    #[must_use] 
    pub const fn lang(&self) -> Option<&'static str> {
        self.lang
    }

    #[inline]
    #[must_use] 
    pub const fn builder(&self) -> &MainBuilder {
        &self.builder
    }

    #[inline]
    pub const fn builder_mut(&mut self) -> &mut MainBuilder {
        &mut self.builder
    }

    #[inline]
    pub const fn paragraph(&mut self) -> &counter::Paragraph {
        &self.owner.paragraph
    }

    #[inline]
    pub const fn paragraph_mut(&mut self) -> &mut counter::Paragraph {
        &mut self.owner.paragraph
    }

    #[inline]
    pub const fn unpack(&mut self) -> UnpackedBuildContext<'_> {
        UnpackedBuildContext {
            builder: &mut self.builder,
            paragraph: &mut self.owner.paragraph,
        }
    }
}

impl Drop for BuildContext<'_> {
    fn drop(&mut self) {
        self.owner.pages.push(Page {
            id: self.id,
            name: self.name,
            main: self.builder.build(),
            lang: self.lang,
        });
    }
}

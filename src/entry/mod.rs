pub mod attachment;
pub mod language;
pub mod qualifier;

pub mod alias {
    pub use super::language::alias as l;
    pub use super::qualifier::alias as q;
}

pub use attachment::Attachment;
use html::{
    inline_text::Abbreviation,
    interactive::{Details, builders::DetailsBuilder, children::SummaryChild},
    text_content::{
        Division,
        ListItem,
        OrderedList,
        UnorderedList,
        builders::ListItemBuilder,
        children::{DivisionChild, ListItemChild},
    },
};
pub use language::Language;
pub use qualifier::Qualifier;

use crate::{counter, html};

pub type Entry<'a> = &'a [Fragment<'a>];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fragment<'a> {
    term: &'a str,
    attachments: &'a [Attachment<'a>],
}

impl<'a> Fragment<'a> {
    #[inline]
    #[must_use]
    pub const fn new(term: &'a str, attachments: &'a [Attachment<'a>]) -> Self {
        Self { term, attachments }
    }

    #[inline]
    #[must_use]
    pub const fn term(&self) -> &'a str {
        self.term
    }

    #[inline]
    #[must_use]
    pub const fn attachments(&self) -> &'a [Attachment<'_>] {
        self.attachments
    }
}

#[inline]
#[must_use]
pub const fn count(entries: &'static [&[(&[Entry<'_>], &'static [Entry<'_>])]]) -> usize {
    let mut count = 0;

    // for page in entries {
    //     count += page.len();
    // }

    let mut idx = 0;
    while let Some(page) = entries.get(idx) {
        count += page.len();
        idx += 1;
    }

    count
}

#[must_use]
pub fn build_entries_noncollapsible<T>(
    entries: &'static [&[(&[Entry<'_>], &'static [Entry<'_>])]],
    offset: usize,
    page: usize,
    title: T,
) -> Division
where
    T: Into<DivisionChild>,
{
    let entry_num = counter::Simple::start(1 + offset);
    let page_num = counter::Page::start(1 + page);

    Division::builder()
        .class("vocabulary")
        .push(title)
        .extend(entry_pages(entries, entry_num, page_num))
        .build()
}

#[must_use]
pub fn build_entries<T>(
    entries: &'static [&[(&[Entry<'_>], &'static [Entry<'_>])]],
    offset: usize,
    page: usize,
    title: T,
) -> DetailsBuilder
where
    T: Into<SummaryChild>,
{
    let entry_num = counter::Simple::start(1 + offset);
    let page_num = counter::Page::start(1 + page);

    let mut builder = Details::builder();
    builder.open(true);
    builder.push(html!(summary(title)).build());
    builder.push(
        Division::builder()
            .class("vocabulary")
            .extend(entry_pages(entries, entry_num, page_num))
            .build(),
    );
    builder
}

pub type Page<'a> = &'a [(&'a [&'a [Fragment<'a>]], &'a [&'a [Fragment<'a>]])];

fn entry_pages(
    entries: &'static [Page<'_>],
    mut entry_num: counter::Counter<counter::label::Simple>,
    mut page_num: counter::Counter<counter::label::Page>,
) -> std::iter::Map<
    std::slice::Iter<'static, Page<'static>>,
    impl FnMut(&'static Page<'static>) -> OrderedList,
> {
    entries.iter().map(move |page| {
        let mut ol = OrderedList::builder();
        ol.id(page_num.increment());

        for (lhs, rhs) in *page {
            let mut li = ListItem::builder();

            if entry_num.current().is_multiple_of(10) {
                li.class("tenth");
            }
            // DO NOT REORDER THESE TWO
            li.id(entry_num.increment());

            for (entries, lang, translate) in [(*lhs, "en", true), (*rhs, "fil-PH", false)] {
                let mut ul = UnorderedList::builder();
                ul.lang(lang);
                ul.translate(translate);

                for entry in entries {
                    let mut li = ListItem::builder();
                    let mut entry = entry.iter();

                    if let Some(first) = entry.next() {
                        append_fragment(&mut li, first);

                        for fragment in entry {
                            li.text(" ");

                            append_fragment(&mut li, fragment);
                        }
                    }

                    ul.push(li.build());
                }

                li.push(ul.build());
            }

            ol.push(li.build());
        }

        ol.build()
    })
}

fn append_fragment(li: &mut ListItemBuilder, fragment: &'static Fragment<'static>) {
    li.text(fragment.term());
    if !fragment.attachments().is_empty() {
        li.text(" (");
        li.extend(itertools::intersperse(
            fragment
                .attachments()
                .iter()
                .map(|attachment| match *attachment {
                    Attachment::Alternative(alternative) => ListItemChild::from(alternative),
                    Attachment::Etymology(etymology) => {
                        let mut ul = UnorderedList::builder();
                        ul.class("etymology");
                        for part in etymology {
                            let mut li = ListItem::builder();
                            li.text(*part);
                            ul.push(li.build());
                        }
                        ListItemChild::from(ul.build())
                    }
                    Attachment::Language(language) => {
                        let mut abbr = Abbreviation::builder();
                        abbr.text(language.abbr());
                        abbr.title(language.name());
                        ListItemChild::from(abbr.build())
                    }
                    Attachment::Gloss(gloss) => gloss.into(),
                    Attachment::Qualifier(qualifier) => {
                        let mut abbr = Abbreviation::builder();
                        abbr.text(qualifier.abbr());
                        abbr.title(qualifier.title());
                        ListItemChild::from(abbr.build())
                    }
                }),
            ", ".into(),
        ));
        li.text(")");
    }
}

#[macro_export]
macro_rules! entries {
    ($([
        $([$($definition:tt)*] => [$($maugnayin:tt)*]),*$(,)?
    ]),*$(,)?) => {
        pub const ENTRIES: &[$crate::entry::Page<'_>] = &[$(
            &[$(
                (
                    &$crate::entries!(@ $($definition)*),
                    &$crate::entries!(@ $($maugnayin)*),
                ),
            )*],
        )*];

        #[allow(unused)]
        pub const COUNT: usize = $crate::entry::count(ENTRIES);
    };

    (@ $($(
        $term:literal $(<$(
            $attachment:ident : $first:tt $($rest:literal)*
        ),+>)?
    )+),+$(,)?) => {
        [$(&[$(
            $crate::entry::Fragment::new(
                $term,
                &[$($(
                    $crate::entries!(attachment @ $attachment : $first $($rest)*),
                )+)?],
            )
        ,)+],)*]
    };

    (attachment @ $attachment:ident : $($ident:ident)+) => {
        $crate::entry::attachment::alias::$attachment ( &[$(
            $crate::entry::alias::$attachment::$ident,
        )+] )
    };

    (attachment @ $attachment:ident : $($literal:literal)+) => {
        $crate::entry::attachment::alias::$attachment ( &[$(
            $literal,
        )+] )
    };
}

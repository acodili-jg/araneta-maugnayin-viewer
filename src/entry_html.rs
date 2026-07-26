use html::inline_text::children::SpanChild;
use html::inline_text::{Abbreviation, Span};
use html::interactive::Details;
use html::interactive::builders::DetailsBuilder;
use html::interactive::children::SummaryChild;
use html::text_content::builders::ListItemBuilder;
use html::text_content::children::{DivisionChild, ListItemChild};
use html::text_content::{Division, ListItem, OrderedList, UnorderedList};

use crate::entry::term::Fragment;
use crate::entry::{Entries, Entry, Language, Page, modifier};
use crate::{counter, html};

pub trait EntriesHtml {
    fn build_entries_noncollapsible<T>(&self, offset: usize, page: usize, title: T) -> Division
    where
        T: Into<DivisionChild>;

    fn build_entries<T>(&self, offset: usize, page: usize, title: T) -> DetailsBuilder
    where
        T: Into<SummaryChild>;
}

impl EntriesHtml for Entries<'static> {
    fn build_entries_noncollapsible<T>(&self, offset: usize, page: usize, title: T) -> Division
    where
        T: Into<DivisionChild>,
    {
        let entry_num = counter::Simple::start(1 + offset);
        let page_num = counter::Page::start(1 + page);

        Division::builder()
            .class("vocabulary")
            .push(title)
            .extend(entry_pages(self, entry_num, page_num))
            .build()
    }

    fn build_entries<T>(&self, offset: usize, page: usize, title: T) -> DetailsBuilder
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
                .extend(entry_pages(self, entry_num, page_num))
                .build(),
        );
        builder
    }
}

fn entry_pages(
    entries: &Entries<'static>,
    mut entry_num: counter::Counter<counter::label::Simple>,
    mut page_num: counter::Counter<counter::label::Page>,
) -> std::iter::Map<
    std::slice::Iter<'static, Page<'static>>,
    impl FnMut(&'static Page<'static>) -> OrderedList,
> {
    entries.pages().iter().map(move |page| {
        let mut ol = OrderedList::builder();
        ol.id(page_num.increment());

        for Entry(lhs, rhs) in page.children() {
            let mut li = ListItem::builder();

            if entry_num.current().is_multiple_of(10) {
                li.class("tenth");
            }
            // DO NOT REORDER THESE TWO
            li.id(entry_num.increment());

            for (mapping, lang, translate) in [(*lhs, "en", true), (*rhs, "fil-PH", false)] {
                let mut ul = UnorderedList::builder();
                ul.lang(lang);
                ul.translate(translate);

                for term in mapping.terms() {
                    let mut li = ListItem::builder();
                    let mut fragments = term.fragments().iter();

                    if let Some(first) = fragments.next() {
                        append_fragment(&mut li, first);

                        for fragment in fragments {
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
    let mut modifiers = fragment
        .modifiers()
        .iter()
        .flat_map(|a| modifier::Left::try_from(*a))
        .map(|modifier| match modifier {
            modifier::Left::Alternative(alternative_display) => alternative_display.to_string(),
            _ => unimplemented!(),
        });

    if let Some(first) = modifiers.next() {
        li.text("(");
        li.text(first);
        for modifier in modifiers {
            li.text(", ");
            li.text(modifier);
        }
        li.text(") ");
    }

    // TODO: evaluate if do we need this.
    if let Some(language) = fragment.language() {
        li.text(language.name());
        li.text(" ");
    }

    li.text(fragment.text());

    let mut modifiers = fragment
        .modifiers()
        .iter()
        .flat_map(|a| modifier::Right::try_from(*a))
        .map(|modifier| match modifier {
            modifier::Right::Alternative(alternative_display) => alternative_display.into(),
            modifier::Right::Etymology(etymology) => match etymology {
                modifier::Etymology::Affix(items) | modifier::Etymology::Blend(items) => {
                    let mut items = items.iter().copied().filter_map(ety_text);
                    let mut builder = Span::builder();
                    if let Some(first) = items.next() {
                        builder.push(first);
                        for item in items {
                            builder.text(" + ");
                            builder.push(item);
                        }
                    }
                    builder.build().into()
                }
                modifier::Etymology::Borrow(items)
                | modifier::Etymology::From(items)
                | modifier::Etymology::Inherited(items) => {
                    let mut items = items.iter().copied().filter_map(ety_text);
                    let mut builder = Span::builder();
                    if let Some(first) = items.next() {
                        builder.push(first);
                        for item in items {
                            builder.text(", ");
                            builder.push(item);
                        }
                    }
                    builder.build().into()
                }
                _ => unimplemented!(),
            },
            modifier::Right::Gloss(gloss) => gloss.into(),
            modifier::Right::PartOfSpeech(part_of_speech) => {
                let mut abbr = Abbreviation::builder();
                abbr.text(part_of_speech.abbr());
                abbr.title(part_of_speech.title());
                ListItemChild::from(abbr.build())
            }
            modifier::Right::Qualifier(qualifier) => qualifier.abbr().map_or_else(
                || ListItemChild::from(qualifier.title()),
                |abbreviation| {
                    let mut abbr = Abbreviation::builder();
                    abbr.text(abbreviation);
                    abbr.title(qualifier.title());
                    ListItemChild::from(abbr.build())
                },
            ),
            _ => unimplemented!(),
        });

    if let Some(first) = modifiers.next() {
        li.text(" (");
        li.push(first);
        for modifier in modifiers {
            li.text(", ");
            li.push(modifier);
        }
        li.text(")");
    }
}

fn ety_text((language, text): (Option<Language>, Option<&'static str>)) -> Option<SpanChild> {
    let language = language.map(|language| {
        let mut abbr = Abbreviation::builder();
        abbr.text(language.abbr());
        abbr.title(language.name());
        abbr.build()
    });

    Some(match (language, text) {
        (None, None) => return None,
        (None, Some(text)) => text.into(),
        (Some(language), None) => language.into(),
        (Some(language), Some(text)) => {
            let mut span = Span::builder();
            span.text(text);
            span.text("( ");
            span.push(language);
            span.text(")");
            span.build().into()
        }
    })
}

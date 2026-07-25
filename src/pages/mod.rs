use html::interactive::builders::DetailsBuilder;

use crate::{entry, html, page::UnpackedBuildContext};

pub mod bungadsabi;
pub mod karaniwang_salita;
pub mod preface;
pub mod sabalak_angkan;
pub mod sinalapat_na_agham;
pub mod sinalapat_na_sipnayan;

pub type Category<'a> = (SubCategory<'a>, &'a [SubCategory<'a>]);

pub type SubCategory<'a> = (bool, &'a str, &'a str, &'a [entry::Page<'a>]);

fn build_categories(categories: &[Category<'static>], mut context: UnpackedBuildContext) {
    let mut details = create_category(
        0,
        0,
        (
            false,
            "Mga Karaniwang Salitang Pang-agham",
            "Common Science Words",
            karaniwang_salita::ENTRIES,
        ),
    );
    details.open(false);
    details.style("counter-reset: vocabulary;".to_string());
    context.builder.push(details.build());

    let (mut offset, mut page) = (karaniwang_salita::COUNT, karaniwang_salita::ENTRIES.len());

    for &category in categories {
        if category.0.0 {
            page -= 1;
        }

        (offset, page) = build_category(&mut context, offset, page, category);
    }
}

fn build_category(
    context: &mut UnpackedBuildContext<'_>,
    mut offset: usize,
    mut page: usize,
    (category, subcategories): Category<'static>,
) -> (usize, usize) {
    let mut details = create_category(offset, page, category);
    details.style(format!("counter-reset: vocabulary {offset};"));

    offset += entry::count(category.3);
    page += category.3.len();

    for &subcategory in subcategories {
        if subcategory.0 {
            page -= 1;
        }

        let mut subdetails = create_subcategory(offset, page, subcategory);
        subdetails.style(format!("counter-reset: vocabulary {offset};"));

        offset += entry::count(category.3);
        page += category.3.len();

        if subcategory.3.is_empty() {
            subdetails.open(false);
        }

        details.push(subdetails.build());
    }

    if category.3.is_empty() && subcategories.is_empty() {
        details.open(false);
    }

    context.builder.push(details.build());
    (offset, page)
}

fn create_category(
    offset: usize,
    page: usize,
    (_, pangalan, name, entries): SubCategory<'static>,
) -> DetailsBuilder {
    entry::build_entries(
        entries,
        offset,
        page,
        html!(h2
            {span (pangalan) translate(false)}
            " "
            {span "("(name)")" lang("en")}
        )
        .build(),
    )
}

fn create_subcategory(
    offset: usize,
    page: usize,
    (_, pangalan, name, entries): SubCategory<'static>,
) -> DetailsBuilder {
    entry::build_entries(
        entries,
        offset,
        page,
        html!(h3
            {span (pangalan) translate(false)}
            " "
            {span "("(name)")" lang("en")}
        )
        .build(),
    )
}

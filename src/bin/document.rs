// Due to the `html` crate's types
#![recursion_limit = "512"]
use std::{io, path::Path};

pub use araneta_maugnayin_viewer::{build_html, html, page, pages, text::Text};
use html::{
    content::{Navigation, builders::NavigationBuilder},
    inline_text::{Anchor, children::AnchorChild},
    root::Body,
    text_content::{ListItem, OrderedList},
};
use itertools::{Itertools, izip};

fn main() {
    let Some(dest) = std::env::args().nth(1) else {
        eprintln!("Missing required destination folder argument.");
        return;
    };

    let document = Document {
        out: &Path::new(&dest).join("html/"),
    };

    if document.initialize() {
        return;
    }

    write_main_pages(&document);

    document.write_page(include_str!("../../index.html"), "../index");
}

struct Document<'a> {
    out: &'a Path,
}

impl Document<'_> {
    fn initialize(&self) -> bool {
        match std::fs::remove_dir_all(self.out) {
            Ok(()) => println!("Cleared html/"),
            Err(e) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
            Err(e) => {
                println!("Unable to clear html/. {e}");
                return true;
            }
        }

        match std::fs::create_dir_all(self.out) {
            Ok(()) => println!("Created html/"),
            Err(e) => println!("Unable to create html/. {e}"),
        }

        false
    }

    fn write_page(&self, html: &str, page_id: &str) {
        let path = self.out.join(format!("{page_id}.html"));
        match std::fs::write(path, html) {
            Ok(()) => println!("Updated html/{page_id}.html"),
            Err(e) => println!("Unable to update html/{page_id}.html. {e}"),
        }
    }
}

fn write_main_pages(document: &Document<'_>) {
    let mut builder = page::Builder::new();
    pages::bungadsabi::bungadsabi(builder.start(
        "bungadsabi",
        Text::new("Bungadsabi", "fil-PH"),
        Some("fil-PH"),
    ));
    pages::preface::preface(builder.start("preface", Text::new("Preface", "en-PH"), Some("en-PH")));
    pages::karaniwang_salita::karaniwang_salita(builder.start(
        "karaniwang-salita",
        Text::new("Mga Karaniwang Salitang Pang-agham", "fil-PH"),
        Some("fil-PH"),
    ));
    pages::sinalapat_na_sipnayan::sinalapat_na_sipnayan(builder.start(
        "sinalapat-na-sipnayan",
        Text::new("Sinalapat na Sipnayan", "fil-PH"),
        Some("fil-PH"),
    ));
    pages::sinalapat_na_agham::sinalapat_na_agham(builder.start(
        "sinalapat-na-agham",
        Text::new("Sinalapat na Agham", "fil-PH"),
        Some("fil-PH"),
    ));
    pages::sabalak_angkan::sabalak_angkan(builder.start(
        "sabalak-angkan",
        Text::new("Sabalak-Angkan", "fil-PH"),
        Some("fil-PH"),
    ));

    let pages = builder.into_pages();
    let page_infos = pages.iter().map(|page| (page.id, page.name)).collect_vec();
    let longest_name = page_infos.iter().max_by_key(|(_, name)| name.text.len());

    for (prev, page, next) in izip!(
        std::iter::once(None).chain(page_infos.iter().map(Some)),
        pages,
        page_infos
            .iter()
            .skip(1)
            .map(Some)
            .chain(std::iter::once(None)),
    ) {
        let mut builder = html!(html
            {head
                {meta charset("utf-8")}
                {meta name("viewport") content("width=device-width, initial-scale=1.0")}
                {title (format!("{} | Araneta Maugnayin Viewer", page.name.text))}
                {link rel("stylesheet") href("../stylesheets/main.css")}
            }
        );

        let mut body = Body::builder();

        let nav = &mut create_page_nav(prev, page.lang, next, longest_name);

        if let Some(nav) = nav {
            build_html!(&mut body; header class("full subcontainer") (nav.build()));
        }

        body.push(page.main);

        if let Some(nav) = nav {
            build_html!(&mut body; footer class("full subcontainer") (nav.build()));
        }

        let mut html = builder.push(body.build()).build();
        html.set_lang(page.lang);

        let html = html.to_string();
        let page_id = page.id;

        document.write_page(&html, page_id);
    }
}

fn create_page_nav(
    prev: Option<&(&'static str, Text<'static>)>,
    page_lang: Option<&'static str>,
    next: Option<&(&'static str, Text<'static>)>,
    longest_name: Option<&(&'static str, Text<'static>)>,
) -> Option<NavigationBuilder> {
    if matches!([prev, next, longest_name], [None, None, None]) {
        return None;
    }

    let mut nav = Navigation::builder();
    nav.class("page-nav");

    let mut ol = OrderedList::builder();

    for (label, curr, class, hidden) in [
        ("Previous Page:", prev, "left", false),
        ("Next Page:", next, "right", false),
        ("Previous Page:", longest_name, "left hidden", true),
        ("Next Page:", longest_name, "right hidden", true),
    ] {
        let mut li = ListItem::builder();
        li.class(class);
        li.aria_hidden(hidden);

        if let Some((id, name)) = curr {
            let mut a = Anchor::builder();

            a.href(format!("{id}.html"));

            build_html!(&mut a;
                (if name.lang.starts_with("en") {
                    AnchorChild::from(label)
                } else {
                    AnchorChild::from(html!(span lang("en") (label)).build())
                })
                {br}
                (name.text)
            );
            if !matches!(page_lang, Some(lang) if lang == name.lang) {
                a.lang(name.lang);
            }

            let mut a = a.build();
            a.set_translate(name.translate);

            li.push(a);
        }

        ol.push(li.build());
    }

    nav.push(ol.build());

    Some(nav)
}

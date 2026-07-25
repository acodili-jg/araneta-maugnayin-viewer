use html::text_content::builders::OrderedListBuilder;

use crate::page::{BuildContext, UnpackedBuildContext};
use crate::{abbr, build_html, html, quote};

use crate::MGA_LARANGAN;

#[inline]
pub fn preface(mut context: BuildContext<'_>) {
    page_vi(context.unpack());
    page_vii(context.unpack());
    page_viii(context.unpack());
}

fn page_vi(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {h1 id("preface") span id("page-vi") "Preface"}

        {p id(context.paragraph.increment())
        "The " {b "Araneta-" (abbr!(NSDB)) " Terminology Program for "
        "Technology Transfer to Rural Areas"} " is a language planning program "
        "financially assisted by the " (for (0, 1, 2, 3) in abbr!(NSDB <>))
        " under its Project No. " {
            span (abbr!(NSDB_GAUF)) " 7706 Spi" translate(false)
        } ". It is designed to compile, as an initial step, nine (9) basic and "
        "interrelated 1000-entry vocabularies or word lists of technical "
        "Pilipino terms and expressions, structured according to the " {span
            "Maugnayin" lang("fil-PH") translate(false)
        } " Approach, in the following "
        "specific technological areas:"}

        {ol [|mut context: OrderedListBuilder| {
            for larangan in MGA_LARANGAN {
                context.push(html!(li a
                    "" (larangan.1)
                    href(larangan.2)
                ).build());
            }
            context
        }]}

        {p id(context.paragraph.increment())
        "All previous word lists or vocabularies in Pilipino, including "
        "those in " {span "Maugnayin Pilipino" lang("fil-PH") translate(false) }
        " published by the "
        {b "Lupon sa Agham" lang("fil-PH") translate(false) } ", a Pilipino "
        "translation committee sponsored by the " (abbr!(UNESCO)) " National "
        "Commission of the Philippines in " {time date_time("1965") "1965"} ", "
        "contained mostly general terms and expressions in the pure sciences; "
        "there was none offering specific, rigorous and systematic entries like"
        " those found in the nine vocabularies compiled under this Program. "
        "These nine word lists are profession-oriented and would make possible "
        "the preparation of adequate textbooks for all the collegiate "
        "technological courses offered in the " {b "Gregorio Araneta "
        "University Foundation"} " as well as for similar courses in other "
        "colleges and universities."}

        {p id(context.paragraph.increment())
        "Family Planning is not a main curricular area but it has been "
        "included as a separate project of the Program because of its relevance"
        " today and because the effectiveness of "
        {span "Maugnayin Pilipino" lang("fil-PH") translate(false) } " was "
        "first demonstrated in this field."}
    );
}

fn page_vii(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {p id(context.paragraph.increment())
        "The general plan of the Program was for each of nine qualified and "
        "experienced faculty members of the University's " {b "Department of "
        "Technical Pilipino"} " to team up with one qualified and experienced "
        "faculty member in " span id("page-vii") "each of the technological or "
        "science institutes of " (abbr!(GAUF)) ". Each team thus formed then "
        "collected, semantically analysed, standardized and tested terms and "
        "expressions in the team's particular field. The nine technological "
        "vocabularies thus produced have been published as convenient booklets "
        "each 9 inches wide, 7.5 inches long and of about 50 pages."}

        {p id(context.paragraph.increment())
        "The work was started on " {time date_time("1978-02-16")
        "February 16, 1978"} " and was completed on "
        {time date_time("1978-07-15") "July 15, 1978"} "."}

        {p id(context.paragraph.increment())
        "Professionals and practitioners have decried the lack of books and "
        "other publications written in technical Pilipino, and this was due to "
        "the lack if systematic and authoritative vocabularies in the different"
        " technological fields. So far, no educational institution aside from "
        "the Gregorio Araneta University Foundation has combined language "
        "expertise with technology and come foreward to supply this important "
        "need. Some of the others prefer to believe that technical vocabularies"
        " will spontaneously evolve in due time. This may be so far in the long"
        " run, but the time frame involved would be unacceptably long; it may "
        "be measured in centuries instead of in school semesters."}

        {p id(context.paragraph.increment())
        "The " {span "Maugnayin" lang("fil-PH") translate(false)} " Approach"
        " utilizes word derivations based on root words, afixes, combining "
        "forms and word formation that are already known, or may be easily "
        "explained, to rural manpower in this contry which has a strong "
        "Malayo-Polynesian language background. The " {span
            "Maugnayin" lang("fil-PH") translate(false)
        } " Approach frowns upon the indiscriminate use of foreign loanwords, "
        "especially those incorporating many Greek and Latin combining forms "
        "that are not easy to explain to those who are not native speakers of "
        "European languages. However, the scientific names of plants and "
        "animals (ex. " {b lang("mul") "Terminalia catappa"} " for the talisay "
        "tree) will be retained in order not to disturb identification."}
    );
}

fn page_viii(context: UnpackedBuildContext<'_>) {
    build_html!(context.builder;
        {p id(context.paragraph.increment())
        "These are four characteristics of the nine vocabularies or word "
        "lists developed in this Program which will " span id("page-viii")
        "facilitate their integration into the whole body of "
        {span "maugnayin" lang("fil-PH") translate(false)} " terminology."}

        {p id(context.paragraph.increment())
        "First, they are all consistent and interrelated with the entries "
        "in the " (quote!(
            "Maugnaying Talasalitaang Pang-agham, Ingles Pilipino"
            lang("fil-PH") translate(false)
        )) " published by the " (abbr!(UNESCO)) "-sponsored "
        {b "Lupon sa Agham" lang("fil-PH") translate(false) } " in "
        {time date_time("1969") "1969"} ". This general science vocabulary "
        "contains some 7,500 words, terms and expressions."}

        {p id(context.paragraph.increment())
        "Second, the first 250 entries of each of the nine booklets are "
        "identical in order to provide all nine profeessional areas with the "
        "same core or nucleus of fundamental science terms."}

        {p id(context.paragraph.increment())
        "Third, of the 350 entries, 200 are the " (quote!("Key Words"))
        " which, in the " {span "Maugnayin" lang("fil-PH") translate(false)}
        " Approach, must be taught to all elementary school children in their "
        "first and second grades. They include such very basic concepts as "
        "number (" {i lang("fil") "bilang"} "), quantity ("
        {i lang("fil") "dami"} "), length (" {i lang("fil") "haba"} "), etc. "
        "Many terms have been included among the " (quote!("Key Words")) " not "
        "primarily because of their meanings but because they clearly "
        "illustrate distinct methods of word formation. By mastering such words"
        " at an early age, elementary school children will indelibly "
        "internalize in their minds the corresponding word morphologies."}

        {p id(context.paragraph.increment())
        "Fourth, all the nine vocabularies are consistent with the "
        "approximately 30,000 "
        {span "maugnayin" lang("fil-PH") translate(false)} " terms and actual "
        "usages catalogued in the " {span
            (for (0, 1) in abbr!(LUNSAG <{b}>)) lang("fil-PH") translate(false)
        } " or " {b "Central Repository of Science Words,"} " a card file being "
        "maintained and continually updated by the Department of Technical "
        "Pilipino of this University."}

        {p id(context.paragraph.increment())
        "The human mind learns fasterm reasons better and thinks more "
        "creatively when there are many pathways of interrelatedness between "
        "and among the words, terms and expressions of the language used. With "
        "this step in the " {b "Araneta-" (abbr!(NSDB)) " Terminology Program "
        "Transfer to Rural Areas,"} " we hope to have taken giant stride on "
        "the road towards the intensive development of science and technology "
        "in this country."}

        {ul li
            {address "Gonsalo del Rosario"}
            {ul
                {li "Project Leader"}
                {li time date_time("1978-07-15") "July 15, 1978"}
            }
        }
    );
}

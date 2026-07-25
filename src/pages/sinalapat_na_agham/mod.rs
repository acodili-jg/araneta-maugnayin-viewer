use crate::entry::entries;
use crate::page::BuildContext;
use crate::{abbr, build_html};

// TODO
const SUBCATS: &[super::Category<'_>] = &[
    (
        (
            false,
            "Ngalan ng mga Bahagi ng Lahatang Agham",
            "Names of the Branches of General Science",
            entries!(),
        ),
        &[],
    ),
    (
        (
            false,
            "Lahatang Katawagan sa Sugnayan",
            "General Terminology in Physics",
            entries!(),
        ),
        &[
            (
                false,
                "Mga Isahin ng Sukat at Sukod",
                "Units of Measurement and Dimension",
                entries!(),
            ),
            (
                false,
                "Mahahalgang Sugnaying Lagiin",
                "Important Physical Constants",
                entries!(),
            ),
        ],
    ),
    (
        (
            false,
            "Lahatang Katawagan sa Kapnayan",
            "General Terminology in Chemistry",
            entries!(),
        ),
        &[
            (
                false,
                "Mga Batas, Simulain at Huna",
                "Laws, Principles and Theories",
                entries!(),
            ),
            (false, "Ang mga Mulangkap", "The Elements", entries!()),
            (
                false,
                "Mga Larawaning Katawagan at Kangalanan ng mga Kapnaying Sangkap",
                "Descriptive Terms and Nomenclature of Chemical Substances",
                entries!(),
            ),
            (
                false,
                "Mga Panambal para sa Kapnayaning Kangalanan",
                "Combining Forms for Chemical Nomenclature",
                entries!(),
            ),
            (
                false,
                "Dagipik at Mulipon ng mga Balangkap",
                "Ions and Radicals of Compounds",
                entries!(),
            ),
        ],
    ),
    (
        (
            false,
            "Lahatang Katawagan sa Haynayan",
            "General Terminology in Biology",
            entries!(),
        ),
        &[],
    ),
];

#[inline]
pub fn sinalapat_na_agham(mut context: BuildContext<'_>) {
    build_html!(context.builder_mut();
        { h1 "Sinalapat na Agham" }
        { h2 "Mga May-Akda" }
        { ul
            {li {address "Porfirio A. Francisco"} ul
                {li "E.T.C.; B.S.E.; M.A. (halangad)"}
                {li "Tanging Pagsasanay: Aghimuing Pilipino"}
                {li "Guro sa Agham: Mababang Paaralang Francisco Benitez"}
                {li "Dalubturo: Kagawaran ng Pilipino, " (for (0, 1, 2, 3) in abbr!(TGPA <>))}
            }
            {li {address "Melencio Y. Santos"} ul
                {li "B.S. Sakahaning Agsikapan (" {
                    span "Bachelor of Science in Agricultural Engineering" lang("en")
                } ") &mdash; " {time date_time("1956") "1956"} " &mdash; " (abbr!(TGPA))}
                {li "M.A. Statistics (Palaulatan) &mdash; Pamantasan ng Pilipinas"}
                {li "Puno: Kagawaran ng Sipnayan-Sugnayan (" {
                    span "Dept. of Math-Physics" lang("en")
                } ") &dash; " (for (0, 1, 2, 3) in abbr!(TGPA <>))}
            }
        }
    );

    super::build_categories(SUBCATS, context.unpack());
}

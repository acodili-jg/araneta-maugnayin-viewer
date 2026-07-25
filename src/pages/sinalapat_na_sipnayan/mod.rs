use crate::entry::entries;
use crate::page::BuildContext;
use crate::{abbr, build_html};

mod bilnuran;
mod palatangkasan;
mod sukgisan;

const SUBCATS: &[super::Category<'_>] = &[
    (
        (
            true,
            "Palatangkasan",
            "Theory of Sets",
            palatangkasan::ENTRIES,
        ),
        &[],
    ),
    ((true, "Bilnuran", "Arithmetic", bilnuran::ENTRIES), &[]),
    ((true, "Sukgisan", "Geometry", sukgisan::ENTRIES), &[]),
    // TODO
    ((false, "Panandaan", "Algebra", entries!()), &[]),
    ((false, "Tasihaan", "Trigonometry", entries!()), &[]),
    ((false, "Tayahan", "Calculus", entries!()), &[]),
    ((false, "Palaulatan", "Statistics", entries!()), &[]),
];

#[inline]
pub fn sinalapat_na_sipnayan(mut context: BuildContext<'_>) {
    build_html!(context.builder_mut();
        { h1 "Sinalapat na Sipnayan" }
        { h2 "Mga May-Akda" }
        { ul
            {li {address "Reynaldo L. Aguilar"} ul
                {li "B.S.E. Kalakhan: Pilipino; Kalit-an: Sugnayan"}
                {li "M.A. Pilipino (halangad)"}
                {li "Kasangguni: " span
                    "Asian-American Bilingual Center Bereley, CA, U.S.A. 1975-76"
                    lang("en") translate(false)
                }
            }
            {li {address "Estrella M. Pedregosa"} ul
                {li "B.S. Kapnaying Agsikapan (" {
                    span "Chemical Engineering" lang("en")
                } ")"}
                {li "B.S. Sipnayan"}
                {li "M.A. Sipnayan (halangad)"}
                {li
                    "Nagtapos ng kaaralan sa pagsasanay sa Aghimuing Panaliksikan ng "
                    (abbr!(NIST_NSDB))
                }
                {li "Kawaksing Puno: Kagawaran ng Sipnayan-Sugnayan (" {
                    span "Math-Physics" lang("en")
                } "), " (abbr!(TGPA))}
                {li "Dalub-ulat (" {
                    span "statistician" lang("en")
                } "): Dalubhasikan, "  (abbr!(TGPA))}
            }
        }
    );

    super::build_categories(SUBCATS, context.unpack());
}

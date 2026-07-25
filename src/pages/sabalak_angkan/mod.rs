use crate::entry::entries;
use crate::page::BuildContext;
use crate::{abbr, build_html};

// TODO
const SUBCATS: &[super::Category<'_>] = &[
    (
        (
            false,
            "Tauhing Balisuplingan",
            "Human Reproduction",
            entries!(),
        ),
        &[
            (
                false,
                "Tauhing Balisuplingan",
                "Human Reproduction",
                entries!(),
            ),
            (false, "Katwanang Pambabae", "Female Anatomy", entries!()),
            (false, "Katwanang Panlalaki", "Male Anatomy", entries!()),
            (
                false,
                "Liksanan at Palasakitan ng Buwanan",
                "Physiology and Pathology of Menstruation",
                entries!(),
            ),
            (false, "Dalubsarian", "Sexology", entries!()),
            (
                false,
                "Balisuplinging Liksanan",
                "Reproductive Physiology",
                entries!(),
            ),
            (
                false,
                "Balatian ng mga Balisuplinging Tatag",
                "Diseases of Reproductive Organs",
                entries!(),
            ),
        ],
    ),
    (
        (
            false,
            "Mga Pamamaraan ng Sabalak-angkan",
            "Family Planning Methods",
            entries!(),
        ),
        &[],
    ),
    (
        (
            false,
            "Asiwa ng Sabalak-angkan",
            "Family Planning Adiminstration",
            entries!(),
        ),
        &[
            (
                false,
                "Mga Uri ng Lingkuran",
                "Types of Services",
                entries!(),
            ),
            (false, "Mga Tatagin", "Establishments", entries!()),
            (
                false,
                "Pinaglilingkurang Santauhan",
                "Population Served",
                entries!(),
            ),
            (
                false,
                "Mga Tagatanggap at Tagagamit ng Sabalak-angkan",
                "Family Planning Acceptors and Users",
                entries!(),
            ),
            (false, "Alaga at Sadalo", "Care and Attendance", entries!()),
            (false, "Tauhan", "Personnel", entries!()),
            (false, "Pagsasanay", "Training", entries!()),
            (false, "Paturuang-lusog", "Health Education", entries!()),
            (false, "Talamitam", "Communication", entries!()),
        ],
    ),
    (
        (false, "Talasantauhan", "Demography", entries!()),
        &[
            (false, "Santauhan", "Population", entries!()),
            (false, "Kasalan", "Nuptiality", entries!()),
            (false, "Pagkakamatay", "Mortality", entries!()),
            (false, "Pagkamapupunlaan", "Fertility", entries!()),
            (false, "Angkan", "Family", entries!()),
            (false, "Sambahayan", "Household", entries!()),
            (false, "Kasakitan", "Morbidity", entries!()),
            (false, "Ngibambayan", "Migration", entries!()),
            (false, "Sabalak-angkan", "Family Planning", entries!()),
        ],
    ),
    ((false, "Galamot", "Drugs", entries!()), &[]),
    ((false, "Sahalga", "Evaluation", entries!()), &[]),
];

#[inline]
pub fn sabalak_angkan(mut context: BuildContext<'_>) {
    build_html!(context.builder_mut();
        { h1 "Sabalak-Angkan" }
        { h2 "Mga May-Akda" }
        { ul
            {li {address "Mggt. Adriano Bartolome"} ul
                {li "M.D. Pamantasan ng Santo Tomas, " time date_time("1943") "1943"}
                {li "Puno: Lingkurang Lusog, " (for (0, 1, 2, 3) in abbr!(<> TGPA))}
                {li "Dalubguro: Sabalak-Angkan, " (abbr!(TGPA))}
            }
            {li {address "Lourdes G. Aguilar"} ul
                {li "B.S.E. Pamantasan ng Santo Tomas, " {time date_time("1967") "1967"} ul
                    {li "Kalakhan: Pilipino"}
                    {li "Kalit-an: Ingles"}
                    {li "M.A.: Halangad"}
                }
                {li "Dalubturo: Kagawaran ng Aghimuing Pilipino Dalubhasaan ng Sining at Agham, " (abbr!(TGPA))}
                {li "Tanging Pag-aaral: Aghimuing Pilipino"}
            }
        }
    );

    super::build_categories(SUBCATS, context.unpack());
}

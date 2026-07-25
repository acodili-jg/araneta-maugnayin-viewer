use crate::{abbr, build_html, page::BuildContext};

// TODO
const SUBCATS: &[super::Category<'_>] = &[
    (
        (false, "Tauhing Balisuplingan", "Human Reproduction", &[]),
        &[
            (false, "Tauhing Balisuplingan", "Human Reproduction", &[]),
            (false, "Katwanang Pambabae", "Female Anatomy", &[]),
            (false, "Katwanang Panlalaki", "Male Anatomy", &[]),
            (false, "Liksanan at Palasakitan ng Buwanan", "Physiology and Pathology of Menstruation", &[]),
            (false, "Dalubsarian", "Sexology", &[]),
            (false, "Balisuplinging Liksanan", "Reproductive Physiology", &[]),
            (false, "Balatian ng mga Balisuplinging Tatag", "Diseases of Reproductive Organs", &[]),
        ],
    ),
    ((false, "Mga Pamamaraan ng Sabalak-angkan", "Family Planning Methods", &[]), &[]),
    (
        (false, "Asiwa ng Sabalak-angkan", "Family Planning Adiminstration", &[]),
        &[
            (false, "Mga Uri ng Lingkuran", "Types of Services", &[]),
            (false, "Mga Tatagin", "Establishments", &[]),
            (false, "Pinaglilingkurang Santauhan", "Population Served", &[]),
            (false, "Mga Tagatanggap at Tagagamit ng Sabalak-angkan", "Family Planning Acceptors and Users", &[]),
            (false, "Alaga at Sadalo", "Care and Attendance", &[]),
            (false, "Tauhan", "Personnel", &[]),
            (false, "Pagsasanay", "Training", &[]),
            (false, "Paturuang-lusog", "Health Education", &[]),
            (false, "Talamitam", "Communication", &[]),
        ],
    ),
    (
        (false, "Talasantauhan", "Demography", &[]),
        &[
            (false, "Santauhan", "Population", &[]),
            (false, "Kasalan", "Nuptiality", &[]),
            (false, "Pagkakamatay", "Mortality", &[]),
            (false, "Pagkamapupunlaan", "Fertility", &[]),
            (false, "Angkan", "Family", &[]),
            (false, "Sambahayan", "Household", &[]),
            (false, "Kasakitan", "Morbidity", &[]),
            (false, "Ngibambayan", "Migration", &[]),
            (false, "Sabalak-angkan", "Family Planning", &[]),
        ],
    ),
    ((false, "Galamot", "Drugs", &[]), &[]),
    ((false, "Sahalga", "Evaluation", &[]), &[]),
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

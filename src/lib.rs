#![feature(const_index)]
#![feature(const_trait_impl)]
// Due to the `html` crate's types
#![recursion_limit = "512"]

pub mod abbr;
pub mod counter;
pub mod entry;
pub mod format;
pub mod page;
pub mod pages;
pub mod text;

pub use text::Text;

pub const MGA_LARANGAN: [(&str, &str, &str); 9] = [
    (
        "Sinalapat na Sipnayan",
        "Applied Mathematics",
        "sinalapat-na-sipnayan.html",
    ),
    (
        "Sinalapat na Agham",
        "Applied Sciences",
        "sinalapat-na-agham.html",
    ),
    ("Sabalak-Angkan", "Family Planning", "sabalak_angkan.html"),
    ("Sakahan", "Agriculture", "sakahan.html"),
    (
        "Sakahaning Agsikapan",
        "Agricultural Engineering",
        "sakahaning-agsikapan.html",
    ),
    ("Palahayupan", "Animal Husbandry", "palahayupan.html"),
    (
        "Palagamutang-Hayop",
        "Veterinarian Medicine",
        "palagamutang_hayop.html",
    ),
    ("Palagubatan", "Forestry", "palagubatan.html"),
    (
        "Pamahayagan at Salathala",
        "Journalism and Piublishing",
        "pamahayagan-at-salathala.html",
    ),
];

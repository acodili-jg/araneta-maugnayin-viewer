macro_rules! abbr_impl {
    ($($name:ident $abbr:literal $title:literal),*$(,)?) => {$(
        #[doc = concat!($abbr, " (", $title, ").")]
        pub const $name: (&'static str, &'static str) = ($abbr, $title);
    )*};
}

#[macro_export]
macro_rules! abbr {
    ($provider:ident <{$elt:ident $($tt:tt)*}> $($rest:tt)*) => {{
        let (abbr, title) = $crate::abbr::$provider;
        (
            $crate::html!(abbr (abbr) $($rest)*).build(),
            $crate::html!($elt " (" (title) $($tt)* ")").build(),
        )
    }};

    ($provider:ident <> $($rest:tt)*) => {{
        let (abbr, title) = $crate::abbr::$provider;
        (
            $crate::html!(abbr (abbr) $($rest)*).build(),
            " (", title, ")",
        )
    }};

    ($provider:ident $($rest:tt)*) => {{
        let (abbr, title) = $crate::abbr::$provider;
        $crate::html!(abbr title(title) (abbr) $($rest)*).build()
    }};

    (<> $provider:ident $($rest:tt)*) => {{
        let (abbr, title) = $crate::abbr::$provider;
        (
            $crate::html!(abbr (title) $($rest)*).build(),
            " (", abbr, ")",
        )
    }};
}

abbr_impl! {
    GAUF "GAUF"
    "Gregorio Araneta University Foundation",

    LUNSAG "LUNSAG"
    "Lunduyang Simpanan ng mga Salitang Aghan",

    NIST_NSDB "NIST-NSDB"
    "National Institute of Science and Technology-National Science and Development Board",

    NSDB "NSDB"
    "National Science and Development Board",

    NSDB_GAUF "NSDB-GAUF"
    "National Science and Development Board-Gregorio Araneta University Foundation",

    PLPA "PLPA"
    "Pambansang Lupon sa Paunlarang Agham",

    TGPA "TGPA"
    "Takaran ng Pamantasang Gregorio Araneta",

    UNESCO "UNESCO"
    "United Nations Educational, Scientific and Cultural Organization",
}

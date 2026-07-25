#[macro_export]
macro_rules! build_html {
    ($builder:expr; $text:literal $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                builder.text($text);
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; build() $($rest:tt)*) => {
        $crate::build_html!(
            $builder.build();
            $($rest)*
        )
    };
    ($builder:expr; $attr:ident($($value:expr),*$(,)?) $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                builder.$attr($($value,)*);
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; $elt:ident $($rest:tt)*) => {{
        #[allow(unused_mut)]
        let mut builder = $builder;
        builder.push($crate::html!($elt $($rest)*).build());
        builder
    }};
    ($builder:expr; { $elt:ident $($tt1:tt)* } $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                builder.push($crate::html!($elt $($tt1)*).build());
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; (for ($($idx:tt),+$(,)?) in $expr:expr) $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                let value = $expr;
                $(builder.push(value.$idx);)+
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; ($expr:expr) $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                builder.push($expr);
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; [in $expr:expr] $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                builder.extend($expr);
                builder
            };
            $($rest)*
        )
    };
    ($builder:expr; [::$root:ident$(:: $path:ident)*!] $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                ::$root$(::$path)*!(builder)
            };
            $($rest)*
        )
    };
    ($builder:expr; [$root:ident$(:: $path:ident)*!] $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                $root$(::$path)*!(builder)
            };
            $($rest)*
        )
    };
    ($builder:expr; [$f:expr] $($rest:tt)*) => {
        $crate::build_html!(
            {
                #[allow(unused_mut)]
                let mut builder = $builder;
                ($f)(builder)
            };
            $($rest)*
        )
    };
    ($builder:expr;) => { $builder };
}

/// This macro builds with [`html`] using a custom syntax.
///
/// ## Syntax
/// - *`element`* *`modifier...`* This is the main syntax.
///   - The *`element`* is the name of the tag. Supports both short and long
///     names (e.g. `Paragraph` and `p`). See [`alias`] for available names.
///   - *`modifier`* A single modifier may be any one of the following:
///     - `{` *`element`* <code>*modifier*...</code> `}` nests a child element
///       with seperate modifiers.
///     - `"text"` adds plain text into this element.
///     - *`attr`* `(` <code>*arg*N...</code> `)` sets an attribute or invokes
///       any of the builder's methods.
///     - `build()` returns the built element, its builder is returned
///       otherwise.
///     - *`element`* <code>*modifier*...</code> nests a child element and
///       transfers the remaining modifiers to that element.
///     - `(` *`expression`* `)` evaluates the expression and pushes it as a
///       child.
///     - `(for (N1, N2, N3...) in` *`expression`* `]` evaluates the expression
///       and indexed into with `N` as a tuple.
///     - `[in` *`expression`* `]` evaluates the expression and as an iterable
///       to add multiple children.
///     - `[` *`function_or_macro`* `]` invokes the function or macro with the
///       current builder as argument. Most often the builder must be returned.
#[macro_export]
macro_rules! html {
    ($elt:ident $($tt:tt)*) => {{
        $crate::build_html!($crate::format::alias::$elt::builder(); $($tt)*)
    }};
}

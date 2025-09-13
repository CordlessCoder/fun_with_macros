#[macro_export]
#[doc(hidden)]
macro_rules! __repeated_print_with_prefix {
    ($delimiter:expr) => {
        |_| Ok(())
    };
    ($delimiter:expr, $val:expr) => {{
        let val = $val;
        move |f: &mut ::core::fmt::Formatter| -> ::core::fmt::Result {
            ::core::fmt::Display::fmt(&$delimiter, f)?;
            ::core::fmt::Display::fmt(&val, f)?;
            Ok(())
        }
    }};
    ($delimiter:expr, $val:expr, $($rest:expr),*) => {{
        let val = $val;
        let rest = $crate::__repeated_print_with_prefix!($delimiter, $($rest),*);
        move |f: &mut ::core::fmt::Formatter| -> ::core::fmt::Result {
            ::core::fmt::Display::fmt(&$delimiter, f)?;
            ::core::fmt::Display::fmt(&val, f)?;
            rest(f)
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __delimited_fmt_inner {
    ($delimiter:expr) => {{
        |_| Ok(())
    }};
    ($delimiter:expr, $first:expr) => {{
        let _delimiter = $delimiter;
        let first = $first;
        move |f: &mut core::fmt::Formatter| -> core::fmt::Result {
            ::core::fmt::Display::fmt(&first, f)
        }
    }};
    ($delimiter:expr, $first:expr, $($val:expr),*) => {{
        let delimiter = $delimiter;
        let first = $first;
        let rest = $crate::__repeated_print_with_prefix!(delimiter, $($val),*);
        move |f: &mut core::fmt::Formatter| -> core::fmt::Result {
            ::core::fmt::Display::fmt(&first, f)?;
            rest(f)
        }
    }};
}

#[macro_export]
macro_rules! delimited_fmt {
    ($($val:expr),*) => {{
        struct DisplayWithFn<F: Fn(&mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result>(F);

        impl<F: Fn(&mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result> ::core::fmt::Display for DisplayWithFn<F> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0(f)
            }
        }
        let print = $crate::__delimited_fmt_inner!($($val),*);
        DisplayWithFn(print)
    }};
}

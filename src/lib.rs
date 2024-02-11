#![feature(macro_metavar_expr)]
#[macro_export]
macro_rules! macros_rule {
    ($ident:ident {
        $(($($rule:tt)*) => ($($expr:tt)*),)+
    }) => ($crate::macros_rule_internal!(
        @parse_macro [$ident] [] $( ($($rule)*) => ($($expr)*); )+
    ););
}

#[macro_export]
macro_rules! macros_rule_internal {
    (@parse_macro
        [$ident:ident]
        [$([($($rule:tt)*) => ($($expr:tt)*)])*]
        @accum ($($parsed_rule:tt)*) => ($($parsed_expr:tt)*);
        $($rest:tt)*
    ) => ($crate::macros_rule_internal!(
        @parse_macro [$ident]
        [$([($($rule)*) => ($($expr)*)])* [($($parsed_rule)*) => ($($parsed_expr)*)]]
        $($rest)*
    ););
    (@parse_macro [$ident:ident] [$([($($rule:tt)*) => ($($expr:tt)*)])+]) => (macro_rules! $ident {
        $(
            ($($rule)*) => ($($expr)*);
        )+
    });
    (@parse_macro
        [$ident:ident]
        [$([($($rule:tt)*) => ($($expr:tt)*)])*]
        $($rest:tt)*
    ) => ($crate::macros_rule_internal!(
        @parse_arm
        [@parse_macro [$ident] [$([($($rule)*) => ($($expr)*)])*] @accum]
        $($rest)*
    ););
    (@parse_arm [$($callback:tt)*] ($($rule:tt)*) => ($($expr:tt)*); $($tail:tt)*) => ($crate::macros_rule_internal!(
        @parse_rule
        [@parse_arm [$($callback)*] @rule] []
        ($($rule)*) => ($($expr)*); $($tail)*
    ););
    (@parse_arm [$($callback:tt)*] @rule ($($rule:tt)*) => ($($expr:tt)*); $($tail:tt)*) => ($crate::macros_rule_internal!(
        @parse_expr
        [@parse_arm [$($callback)*] @expr ($($rule)*) =>] []
        ($($expr)*); $($tail)*
    ););
    (@parse_arm [$($callback:tt)*] @expr ($($rule:tt)*) => ($($expr:tt)*); $($tail:tt)*) => ($crate::macros_rule_internal!(
        $($callback)* ($($rule)*) => ($($expr)*); $($tail)*
    ););
    (@parse_rule [$($callback:tt)*] [$($buffer:tt)*] ($tt:tt $($rest:tt)*) $($tail:tt)*) => ($crate::macros_rule_internal!(
        @parse_rule [$($callback)*] [$($buffer)* $tt] ($($rest)*) $($tail)*
    ););
    (@parse_rule [$($callback:tt)*] [$($buffer:tt)*] () $($tail:tt)*) => ($crate::macros_rule_internal!(
        $($callback)* ($($buffer)*) $($tail)*
    ););
    (@parse_expr [$($callback:tt)*] [$($buffer:tt)*] ($tt:tt $($rest:tt)*) $($tail:tt)*) => ($crate::macros_rule_internal!(
        @parse_expr [$($callback)*] [$($buffer)* $tt] ($($rest)*) $($tail)*
    ););
    (@parse_expr [$($callback:tt)*] [$($buffer:tt)*] () $($tail:tt)*) => ($crate::macros_rule_internal!(
        $($callback)* ($($buffer)*) $($tail)*
    ););
}

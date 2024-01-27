#[macro_export]
macro_rules! macros_rule {
    ($name:ident {
        $(
            ($($arm:tt)*) => ($($out:tt)*),
        )+
    }) => {
        macro_rules! $name {
            $(
                ($($arm)*) => ($($out)*);
            )+
        }
    };
}

#[macro_export]
macro_rules! decl_simple_type_enum {
    (pub enum $name:ident {
        $($variant:ident = $str_value:expr),*,
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $name {
            $($variant),*,
        }

        impl ::std::str::FromStr for $name {
            type Err = ParseEnumError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $($str_value => Ok($name::$variant)),*,
                    _ => Err(Self::Err::new(stringify!($name))),
                }
            }
        }
    };
}

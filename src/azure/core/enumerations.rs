#[derive(Debug)]
pub enum ParsingError {
    ElementNotFound(String),
}

/// use as
/// create_enum!(SecondCollection, (Pollo, "Pollo"), (Bianco, "Bianco"), (Giallo, "Giallo"));
macro_rules! create_enum {
    ($en:ident, $(($na:ident, $x:expr)), *) => (
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub enum $en {
            $(
                $na,
            )*
        }

        impl FromStr for $en {
            type Err = enumerations::ParsingError;

            fn from_str(s: &str) -> Result<$en, enumerations::ParsingError> {
                match s {
                    $(
                        $x => Ok($en::$na),
                    )*
                    _ => Err(enumerations::ParsingError::ElementNotFound(s.to_owned())),
                }
            }
        }

        impl fmt::Display for $en {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        &$en::$na => write!(f, "{}", $x),
                    )*
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use azure::core::enumerations;
    use std::str::FromStr;
    use std::fmt;

    create_enum!(Colors, (Black, "Black"), (White, "White"), (Red, "Red"));
    create_enum!(ColorsMonochrome, (Black, "Black"), (White, "White"));

    #[test]
    fn test_color_Parse_1() {
        let color = "Black".parse::<Colors>().unwrap();
        assert_eq!(Colors::Black, color);
    }

    #[test]
    fn test_color_Parse_2() {
        let color = "White".parse::<ColorsMonochrome>().unwrap();
        assert_eq!(ColorsMonochrome::White, color);
    }

    #[test]
    #[should_panic(expected = "ElementNotFound(\"Red\")")]
    fn test_color_Parse_Err_1() {
        let color_mono = "Red".parse::<ColorsMonochrome>().unwrap();
    }
}

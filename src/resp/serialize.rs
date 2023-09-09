use crate::resp::resp_parsing_utils::END_OF_LINE;

use super::resp_parsing_utils::append_crlf;

#[derive(PartialEq)]
pub enum InputVariants {
    NumberVariant(i32),
    StringVariant(String),
    ErrorVariant(String),
    StringVariantArr(Vec<String>),
    Nullish,
}

pub fn serialize(input: InputVariants) -> String {
    match input {
        InputVariants::NumberVariant(number) => {
            append_crlf(concat_string!(":", number.to_string()))
        }
        InputVariants::StringVariant(string) => {
            if string.starts_with('+') {
                append_crlf(string)
            } else {
                concat_string!(
                    append_crlf(concat_string!("$", string.len().to_string())),
                    append_crlf(&string)
                )
            }
        }
        InputVariants::ErrorVariant(string) => append_crlf(string),
        InputVariants::StringVariantArr(string_arr) => {
            let serialized_items: Vec<String> = string_arr
                .iter()
                .map(|item| serialize(InputVariants::StringVariant(item.clone())))
                .collect();

            concat_string!(
                "*",
                serialized_items.len().to_string(),
                END_OF_LINE,
                serialized_items.join("")
            )
        }
        _ => append_crlf("$-1"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_null_to_bulk() {
        assert_eq!(
            serialize(InputVariants::Nullish),
            append_crlf("$-1".to_string())
        )
    }

    #[test]
    fn should_serialize_bulk_string() {
        assert_eq!(
            serialize(InputVariants::StringVariant("PING".to_string())),
            "$4\r\nPING\r\n"
        )
    }

    #[test]
    fn should_serialize_arr_of_bulk_string() {
        assert_eq!(
            serialize(InputVariants::StringVariantArr(vec!(
                "echo".to_string(),
                "hello world".to_string()
            ))),
            "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n"
        )
    }

    #[test]
    fn should_serialize_simple_string() {
        assert_eq!(
            serialize(InputVariants::StringVariant("+PONG".to_string())),
            append_crlf("+PONG".to_string())
        )
    }

    #[test]
    fn should_serialize_integer() {
        assert_eq!(
            serialize(InputVariants::NumberVariant(1)),
            append_crlf(":1".to_string())
        )
    }
}

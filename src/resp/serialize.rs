use std::borrow::Cow;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum InputVariants {
    NumberVariant(i32),
    StringVariant(String),
    ErrorVariant(String),
    StringVariantArr(Vec<String>),
    Nullish,
}

pub fn serialize(input: InputVariants) -> Cow<'static, str> {
    match input {
        InputVariants::NumberVariant(number) => {
            Cow::Owned(concat_string!(":", number.to_string(), "\r\n"))
        }
        InputVariants::StringVariant(string) if string.starts_with('+') => {
            Cow::Owned(concat_string!(string, "\r\n"))
        }
        InputVariants::StringVariant(string) => Cow::Owned(concat_string!(
            "$",
            string.len().to_string(),
            "\r\n",
            string,
            "\r\n"
        )),
        InputVariants::ErrorVariant(string) => Cow::Owned(concat_string!(string, "\r\n")),
        InputVariants::StringVariantArr(string_arr) => {
            let serialized_items: Vec<String> = string_arr
                .iter()
                .map(|item| serialize(InputVariants::StringVariant(item.clone())).into_owned())
                .collect();

            Cow::Owned(concat_string!(
                "*",
                serialized_items.len().to_string(),
                "\r\n",
                serialized_items.concat().to_string()
            ))
        }
        _ => Cow::Borrowed("$-1\r\n"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_null_to_bulk() {
        assert_eq!(serialize(InputVariants::Nullish), "$-1\r\n".to_string())
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
            "+PONG\r\n".to_string()
        )
    }

    #[test]
    fn should_serialize_integer() {
        assert_eq!(
            serialize(InputVariants::NumberVariant(1)),
            ":1\r\n".to_string()
        )
    }
}

use std::str::Split;

use super::{
    deserialize::{deserialize, RespResponse},
    error::ErrMessages,
};

pub static END_OF_LINE: &str = "\r\n";

fn split_data(serialized_input: &str) -> (String, String) {
    let mut head_and_tail: Split<'_, &str> = serialized_input.split(END_OF_LINE);
    let head = head_and_tail.next().unwrap_or_default().to_string();
    let tail: Vec<&str> = head_and_tail.collect();
    let joined_tail = tail.join(END_OF_LINE);
    (head, joined_tail)
}

pub fn read_bulk_string(serialized_input: &str) -> Result<(String, String), ErrMessages> {
    let (string_length, value) = split_data(serialized_input);
    let parsed_string_length = string_length
        .parse::<usize>()
        .map_err(|err| ErrMessages::ParseError(err.to_string()))?;

    let bulk_string_value: String = value.chars().take(parsed_string_length).collect();
    let remaining_tail: String = value.chars().skip(parsed_string_length + 2).collect();
    Ok((bulk_string_value, remaining_tail))
}

pub fn read_array(data: &str) -> Result<(Vec<String>, String), ErrMessages> {
    let (arr_length, mut remaining_data) = split_data(data);
    let count = arr_length
        .parse::<usize>()
        .map_err(|err| ErrMessages::UnknownInput(err.to_string()))?;

    let mut items: Vec<String> = Vec::new();

    for _ in 0..count {
        let parsed_item = deserialize(&remaining_data)?;
        match parsed_item {
            RespResponse::TupleVariant(head, tail) => {
                remaining_data = tail;
                items.push(head);
            }
            RespResponse::VecVariant(_, _) => {
                return Err(ErrMessages::UnexpectedVariant);
            }
        }
    }

    Ok((items, remaining_data))
}

pub fn read_simple_string(serialized_input: &str) -> (String, String) {
    split_data(serialized_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_append_crlf() {
        assert_eq!("world\r\n", "world\r\n".to_string());
    }

    #[test]
    fn should_split_data() {
        assert_eq!(
            split_data("$5\r\nworld\r\n"),
            ("$5".to_string(), "world\r\n".to_string())
        );
    }

    #[test]
    fn should_deserialize_bulk_string() {
        let dollar_stripped_input = "5\r\nworld\r\n";
        assert_eq!(
            read_bulk_string(dollar_stripped_input).unwrap().0,
            "world".to_string()
        );
    }
    #[test]
    fn should_not_deserialize_bulk_string() {
        let dollar_stripped_input = "-1\r\nworld\r\n";
        assert!(read_bulk_string(dollar_stripped_input).is_err(),);
    }

    #[test]
    fn should_deserialize_simple_string() {
        let dollar_stripped_input = "OK\r\n";
        assert_eq!(
            read_simple_string(dollar_stripped_input).0,
            "OK".to_string()
        );
    }
}

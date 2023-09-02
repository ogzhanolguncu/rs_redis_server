//$5\r\nworld\r\n

use std::str::Split;

#[derive(Debug, PartialEq)]
pub enum ErrMessages {
    MissingBulkString,
    EmptyInput,
}

impl ErrMessages {
    pub fn default_message(&self) -> &str {
        match self {
            ErrMessages::MissingBulkString => "Bulk string cannot be empty or null!",
            ErrMessages::EmptyInput => "Input cannot be empty!",
        }
    }
}

static END_OF_LINE: &str = "\r\n";

fn split_data(serialized_input: &str) -> (String, String) {
    let mut head_and_tail: Split<'_, &str> = serialized_input.split(END_OF_LINE);
    let head = head_and_tail.next().unwrap_or_default().to_string();
    let tail: Vec<&str> = head_and_tail.collect();
    let joined_tail = tail.join(END_OF_LINE);
    (head, joined_tail)
}

/// Bulk string should receieve this example text\
/// **"5\r\nworld\r\n"**\
/// and return\
/// **"world"**\
/// Note: Dollar sign should be stripped otherwise parsing will fail and function will return empty string
fn read_bulk_string(serialized_input: &str) -> (String, String) {
    let (string_length, value) = split_data(serialized_input);
    let parsed_string_length = string_length.parse::<usize>().unwrap_or_default(); // Handle length value being 0
    let bulk_string_value: String = value.chars().skip(0).take(parsed_string_length).collect();
    let remaining_tail: String = value
        .chars()
        .skip(parsed_string_length)
        .take(parsed_string_length + 2)
        .collect();
    (bulk_string_value, remaining_tail)
}

/// Simple string should receieve this example text\
/// **"+OK"**\
/// and return\
/// **"OK"**\
/// Note: Dollar sign should be stripped otherwise parsing will fail and function will return empty string
fn read_simple_string(serialized_input: &str) -> (String, String) {
    split_data(serialized_input)
}

pub fn parse_resp(serialized_input: &str) -> Result<(char, String), ErrMessages> {
    if serialized_input == "$-1\r\n" {
        return Err(ErrMessages::MissingBulkString);
    }
    if serialized_input.is_empty() {
        return Err(ErrMessages::EmptyInput);
    }
    let (first_char, tail) = (
        serialized_input.chars().next().unwrap_or_default(),
        &serialized_input[1..],
    );
    Ok((first_char, tail.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

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
            read_bulk_string(dollar_stripped_input).0,
            "world".to_string()
        );
    }

    #[test]
    fn should_deserialize_simple_string() {
        let dollar_stripped_input = "OK\r\n";
        assert_eq!(
            read_simple_string(dollar_stripped_input).0,
            "OK".to_string()
        );
    }

    #[test]
    fn should_separate_first_char_from_input_string() {
        assert_eq!(parse_resp("+OK\r\n").unwrap(), ('+', "OK\r\n".to_string()));
    }

    #[test]
    fn should_return_error_when_empty_bulk_passed() {
        assert_eq!(
            parse_resp("$-1\r\n").unwrap_err(),
            ErrMessages::MissingBulkString
        );
    }
    #[test]
    fn should_return_error_when_empty_input_passed() {
        assert_eq!(parse_resp("").unwrap_err(), ErrMessages::EmptyInput)
    }
}
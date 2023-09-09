use super::{
    error::ErrMessages,
    resp_parsing_utils::{read_array, read_bulk_string, read_simple_string},
};

#[derive(Debug, PartialEq)]
pub enum RespResponse {
    TupleVariant(String, String),
    VecVariant(Vec<String>, String),
}

pub fn deserialize(serialized_input: &str) -> Result<RespResponse, ErrMessages> {
    if serialized_input == "$-1\r\n" {
        return Err(ErrMessages::MissingBulkString);
    }
    if serialized_input.is_empty() {
        return Err(ErrMessages::EmptyInput);
    }
    let (first_char, serialized_input_input) = (
        serialized_input.chars().next().unwrap_or_default(),
        &serialized_input[1..],
    );

    match first_char {
        '+' | '-' => {
            let (head, tail) = read_simple_string(serialized_input_input);
            Ok(RespResponse::TupleVariant(head, tail))
        }
        '$' => read_bulk_string(serialized_input_input)
            .map(|(head, tail)| RespResponse::TupleVariant(head, tail)),

        '*' => read_array(serialized_input_input)
            .map(|(head, tail)| RespResponse::VecVariant(head, tail)),
        details => Err(ErrMessages::UnknownInput(details.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_with_parse_resp() {
        assert_eq!(
            deserialize("$7\r\nCOMMAND").unwrap(),
            RespResponse::TupleVariant("COMMAND".to_string(), "".to_string())
        );
    }

    #[test]
    fn should_return_error_when_empty_bulk_passed() {
        assert_eq!(
            deserialize("$-1\r\n").unwrap_err(),
            ErrMessages::MissingBulkString
        );
    }
    #[test]
    fn should_return_error_when_empty_input_passed() {
        assert_eq!(deserialize("").unwrap_err(), ErrMessages::EmptyInput)
    }

    #[test]
    fn todoo() {
        assert_eq!(
            deserialize("+PING"),
            Ok(RespResponse::TupleVariant(
                "PING".to_string(),
                "".to_string()
            ))
        )
    }

    #[test]
    fn should_deserialize_array() {
        assert_eq!(
            deserialize("*2\r\n$7\r\nCOMMAND\r\n$4\r\nDOCS\r\n").unwrap(),
            RespResponse::VecVariant(
                vec!["COMMAND".to_string(), "DOCS".to_string()],
                String::new()
            )
        )
    }
}

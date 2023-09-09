use std::borrow::Cow;

use super::utils::throw_err_if_num_of_args_wrong;
use crate::{
    deserialize, resp::deserialize::RespResponse, resp::serialize::InputVariants, serialize,
};

pub fn handle_command(human_readable: Cow<'_, str>) -> String {
    let command = deserialize(&human_readable);

    match command {
        Ok(deserialized_command) => match deserialized_command {
            RespResponse::VecVariant(commands, _) => {
                if let Some(command) = commands.first().map(|s| s.to_lowercase()) {
                    let args = &commands[1..];

                    match command.as_str() {
                        "command" => serialize(InputVariants::Nullish),
                        "ping" => serialize(InputVariants::StringVariant("pong".to_string())),
                        "echo" => match args.len() {
                            1 => serialize(InputVariants::StringVariant(args[0].clone())),
                            _ => throw_err_if_num_of_args_wrong("echo"),
                        },
                        unknown_command => serialize(InputVariants::StringVariant(concat_string!(
                            "-ERR command not supported ",
                            unknown_command
                        ))),
                    }
                } else {
                    serialize_error("-ERR Commands array is empty")
                }
            }
            _ => serialize_error("-ERR Unsupported RESP type"),
        },
        Err(err) => {
            println!("{}", err);
            serialize_error("-ERR Failed to deserialize")
        }
    }
}

fn serialize_error(message: &str) -> String {
    println!("{}", message);
    serialize(InputVariants::ErrorVariant(message.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_serialized_pong() {
        let input = Cow::Borrowed("*1\r\n$4\r\nPING\r\n");
        assert_eq!("$4\r\npong\r\n", handle_command(input))
    }
    #[test]
    fn should_echo_hello_world() {
        let input = Cow::Borrowed("*2\r\n$4\r\nECHO\r\n$11\r\nHELLO WORLD\r\n");
        assert_eq!("$11\r\nHELLO WORLD\r\n", handle_command(input));
    }

    #[test]
    fn should_return_error_when_echo_have_too_many_args() {
        let input = Cow::Borrowed("*3\r\n$4\r\nECHO\r\n$4\r\nHEHE\r\n$4\r\nHEHE\r\n");
        assert_eq!(
            throw_err_if_num_of_args_wrong("echo"),
            handle_command(input)
        );
    }

    #[test]
    fn should_return_error_when_fail_to_deserialize() {
        let input = Cow::Borrowed("*1\r\nSILLY");
        assert_eq!(
            serialize_error("-ERR Failed to deserialize"),
            handle_command(input)
        );
    }

    #[test]
    fn should_return_error_when_unknown_command() {
        let input = Cow::Borrowed("*2\r\n$5\r\nECHOO\r\n$2\r\nRT\r\n");
        assert_eq!(
            serialize(InputVariants::StringVariant(concat_string!(
                "-ERR command not supported ",
                "echoo"
            ))),
            handle_command(input)
        );
    }
}

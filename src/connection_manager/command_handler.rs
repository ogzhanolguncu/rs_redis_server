use super::utils::err_if_num_of_args_wrong;
use crate::serialize;
use crate::{deserialize, resp::deserialize::RespResponse, resp::serialize::InputVariants};

pub fn handle_command(human_readable: std::borrow::Cow<'_, str>) -> String {
    let command = deserialize(&human_readable);

    if let Ok(deserialized_command) = command {
        match deserialized_command {
            RespResponse::VecVariant(commands, _) => {
                if let Some(command) = commands.first().map(|s| s.to_lowercase()) {
                    let args = &commands[1..];

                    match command.as_str() {
                        "command" => serialize(InputVariants::Nullish),
                        "ping" => serialize(InputVariants::StringVariant("+pong".to_string())),
                        "echo" => match args.len() {
                            1 => serialize(InputVariants::StringVariant(args[0].clone())),
                            _ => err_if_num_of_args_wrong("echo"),
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
        }
    } else {
        serialize_error("-ERR Failed to deserialize")
    }
}

fn serialize_error(message: &str) -> String {
    println!("{}", message);
    serialize(InputVariants::ErrorVariant(message.to_string()))
}

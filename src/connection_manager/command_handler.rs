use super::utils::err_if_num_of_args_wrong;
use crate::serialize;
use crate::{deserialize, resp::deserialize::RespResponse, resp::serialize::InputVariants};

pub fn handle_command(human_readable: std::borrow::Cow<'_, str>) -> String {
    let command = deserialize(&human_readable);

    if let Ok(deserialized_command) = command {
        match deserialized_command {
            RespResponse::VecVariant(commands, _) => {
                let first_element: &String = commands.first().unwrap();
                let args = &commands[1..];

                match first_element.to_lowercase().as_str() {
                    "command" => return serialize(InputVariants::Nullish),
                    "ping" => return serialize(InputVariants::StringVariant("+pong".to_string())),
                    "echo" => {
                        if args.len() != 1 {
                            err_if_num_of_args_wrong("echo")
                        } else {
                            serialize(InputVariants::StringVariant(args[0].clone()))
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    } else {
        todo!()
    }
}

use crate::resp::serialize::{serialize, InputVariants};

use super::utils::throw_err_if_num_of_args_wrong;

pub fn handle_echo(args: &[String]) -> String {
    match args.len() {
        1 => serialize(InputVariants::StringVariant(args[0].clone())),
        _ => throw_err_if_num_of_args_wrong("echo"),
    }
}

pub fn handle_ping() -> String {
    serialize(InputVariants::StringVariant("pong".to_string()))
}

pub fn ignore_command() -> String {
    serialize(InputVariants::Nullish)
}

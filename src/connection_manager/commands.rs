use crate::{
    resp::serialize::{serialize, InputVariants},
    store::db::Cache,
};

use super::utils::{serialize_error, throw_err_if_num_of_args_wrong};

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

pub fn handle_get(args: &[String], cache: &Cache) -> String {
    if let Some(key) = args.get(0) {
        if let Some(response) = cache.get(key) {
            serialize(InputVariants::StringVariant(response))
        } else {
            serialize_error("-ERR Missing or Expired value")
        }
    } else {
        serialize_error("-ERR Invalid GET arguments")
    }
}

pub fn handle_set(args: &[String], cache: &Cache) -> String {
    if let (Some(key), Some(value)) = (args.get(0), args.get(1)) {
        cache.set(key.clone(), value.clone());
        serialize(InputVariants::StringVariant("+OK".to_string()))
    } else {
        serialize_error("-ERR Invalid SET arguments")
    }
}

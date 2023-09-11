use std::borrow::Cow;

use super::{
    commands::{handle_echo, handle_get, handle_ping, handle_set, ignore_command},
    utils::serialize_error,
};
use crate::{deserialize, resp::deserialize::RespResponse, store::db::Cache};

pub fn handle_command(human_readable: Cow<'_, str>, cache: &Cache) -> String {
    let command = deserialize(&human_readable);

    match command {
        Ok(deserialized_command) => match deserialized_command {
            RespResponse::VecVariant(commands, _) => {
                if let Some(command) = commands.first().map(|s| s.to_lowercase()) {
                    let args = &commands[1..];

                    match command.as_str() {
                        "command" => ignore_command(),
                        "ping" => handle_ping(),
                        "echo" => handle_echo(args),
                        "set" => handle_set(args, cache),
                        "get" => handle_get(args, cache),
                        unknown_command => {
                            let message = "-unknown command '".to_owned() + unknown_command + "'";
                            serialize_error(message.as_str())
                        }
                    }
                } else {
                    serialize_error("-commands array is empty")
                }
            }
            _ => serialize_error("-unsupported RESP type"),
        },
        Err(err) => {
            println!("{}", err);
            serialize_error("-failed to deserialize")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{
        connection_manager::utils::throw_err_if_num_of_args_wrong,
        resp::serialize::{serialize, InputVariants},
    };

    use super::*;

    #[test]
    fn should_return_serialized_pong() {
        let input = Cow::Borrowed("*1\r\n$4\r\nPING\r\n");
        assert_eq!("$4\r\npong\r\n", handle_command(input, &Cache::new()))
    }
    #[test]
    fn should_echo_hello_world() {
        let input = Cow::Borrowed("*2\r\n$4\r\nECHO\r\n$11\r\nHELLO WORLD\r\n");
        assert_eq!(
            "$11\r\nHELLO WORLD\r\n",
            handle_command(input, &Cache::new())
        );
    }

    #[test]
    fn should_return_error_when_echo_have_too_many_args() {
        let input = Cow::Borrowed("*3\r\n$4\r\nECHO\r\n$4\r\nHEHE\r\n$4\r\nHEHE\r\n");
        assert_eq!(
            throw_err_if_num_of_args_wrong("echo"),
            handle_command(input, &Cache::new())
        );
    }

    #[test]
    fn should_return_error_when_fail_to_deserialize() {
        let input = Cow::Borrowed("*1\r\nSILLY");
        assert_eq!(
            serialize_error("-failed to deserialize"),
            handle_command(input, &Cache::new())
        );
    }

    #[test]
    fn should_return_error_when_unknown_command() {
        let input = Cow::Borrowed("*2\r\n$5\r\nECHOO\r\n$2\r\nRT\r\n");
        assert_eq!(
            serialize_error(format!("-unknown command '{}'", "echoo").as_str()),
            handle_command(input, &Cache::new())
        );
    }

    #[test]
    fn should_return_set_to_cache() {
        let input = Cow::Borrowed("*3\r\n$3\r\nset\r\n$4\r\nname\r\n$12\r\nWizard of Oz\r\n");
        assert_eq!(
            serialize(InputVariants::StringVariant("+OK".to_string())),
            handle_command(input, &Cache::new())
        );
    }

    #[test]
    fn should_return_get_to_cache() {
        let (key, value) = ("name".to_string(), "Wizard of Oz".to_string());
        let cache = Cache::new();
        cache.set(key, value);

        let input = Cow::Borrowed("*2\r\n$3\r\nget\r\n$4\r\nname\r\n");
        assert_eq!(
            serialize(InputVariants::StringVariant("Wizard of Oz".to_string())),
            handle_command(input, &cache)
        );
    }

    #[test]
    fn should_set_with_expiration_and_retrive_expired() {
        let cache = Cache::new();
        let set_input = Cow::Borrowed(
            "*5\r\n$3\r\nset\r\n$4\r\nname\r\n$12\r\nWizard of Oz\r\n$2\r\nEX\r\n$1\r\n3",
        );
        let get_input = Cow::Borrowed("*2\r\n$3\r\nget\r\n$4\r\nname\r\n");
        handle_command(set_input, &cache);
        thread::sleep(Duration::from_secs(4));

        assert_eq!(
            serialize(InputVariants::StringVariant("+(nil)".to_string())),
            handle_command(get_input, &cache)
        );
    }

    #[test]
    fn should_set_with_unknown_expiration_variant() {
        let cache = Cache::new();
        let set_input = Cow::Borrowed(
            "*5\r\n$3\r\nset\r\n$4\r\nname\r\n$12\r\nWizard of Oz\r\n$6\r\nEXATAT\r\n$1\r\n3",
        );
        assert_eq!(
            serialize_error("-unknown SET variant"),
            handle_command(set_input, &cache)
        );
    }

    #[test]
    fn should_set_with_unparseable_value() {
        let cache = Cache::new();
        let set_input = Cow::Borrowed(
            "*5\r\n$3\r\nset\r\n$4\r\nname\r\n$12\r\nWizard of Oz\r\n$6\r\nEXATAT\r\n$3\r\nAAA",
        );
        assert_eq!(
            serialize_error("-invalid SET expiration"),
            handle_command(set_input, &cache)
        );
    }
}

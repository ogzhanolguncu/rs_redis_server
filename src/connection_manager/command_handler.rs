use std::borrow::Cow;

use super::{
    commands::{
        handle_decr, handle_del, handle_echo, handle_exists, handle_get, handle_incr, handle_ping,
        handle_set, ignore_command,
    },
    utils::serialize_error,
};
use crate::{deserialize, resp::deserialize::RespResponse, store::db::Cache};

pub fn handle_command(human_readable: Cow<'_, str>, cache: &Cache) -> Cow<'static, str> {
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
                        "exists" => handle_exists(args, cache),
                        "del" => handle_del(args, cache),
                        "incr" => handle_incr(args, cache),
                        "decr" => handle_decr(args, cache),
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

    #[test]
    fn should_return_existing_values() {
        let cache = Cache::new();
        cache.set("name".to_string(), "name_val".to_string());
        cache.set("name1".to_string(), "name_val_1".to_string());
        cache.set("name2".to_string(), "name_val_2".to_string());
        let input =
            Cow::Borrowed("*4\r\n$6\r\nexists\r\n$4\r\nname\r\n$5\r\nname1\r\n$5\r\nname2\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(3)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_return_zero_if_not_exists() {
        let cache = Cache::new();
        let input = Cow::Borrowed("*2\r\n$6\r\nexists\r\n$4\r\nname\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(0)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_del_values() {
        let cache = Cache::new();
        cache.set("name".to_string(), "name_val".to_string());
        cache.set("name1".to_string(), "name_val_1".to_string());
        cache.set("name2".to_string(), "name_val_2".to_string());
        let input =
            Cow::Borrowed("*4\r\n$3\r\ndel\r\n$4\r\nname\r\n$5\r\nname1\r\n$5\r\nname2\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(3)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_return_zero_if_cant_remove() {
        let cache = Cache::new();
        let input = Cow::Borrowed("*2\r\n$3\r\ndel\r\n$4\r\nname\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(0)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_create_when_incr_if_not_exists() {
        let cache = Cache::new();
        let input = Cow::Borrowed("*2\r\n$4\r\nincr\r\n$5\r\nmykey\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(1)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_create_incr_if_exists() {
        let cache = Cache::new();
        cache.set("mykey".to_string(), 11.to_string());
        let input = Cow::Borrowed("*2\r\n$4\r\nincr\r\n$5\r\nmykey\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(12)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_create_when_decr_if_not_exists() {
        let cache = Cache::new();
        let input = Cow::Borrowed("*2\r\n$4\r\ndecr\r\n$5\r\nmykey\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(-1)),
            handle_command(input, &cache)
        )
    }

    #[test]
    fn should_create_decr_if_exists() {
        let cache = Cache::new();
        cache.set("mykey".to_string(), 11.to_string());
        let input = Cow::Borrowed("*2\r\n$4\r\ndecr\r\n$5\r\nmykey\r\n");
        assert_eq!(
            serialize(InputVariants::NumberVariant(10)),
            handle_command(input, &cache)
        )
    }
}

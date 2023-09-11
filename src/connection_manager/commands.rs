use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
            serialize(InputVariants::StringVariant("+(nil)".to_string()))
        }
    } else {
        serialize_error("-invalid GET arguments")
    }
}

pub fn handle_set(args: &[String], cache: &Cache) -> String {
    let (expiration_variant, expiration_time) = (args.get(2), args.get(3));

    if expiration_variant.is_some() && expiration_time.is_some() {
        let exp_variant = expiration_variant.unwrap();
        let exp_time = expiration_time.unwrap().parse::<u64>();

        if let Ok(time) = exp_time {
            match exp_variant.clone().as_str() {
                "EX" => handle_set_with_expiration(args, cache, Duration::from_secs(time)),
                "PX" => handle_set_with_expiration(args, cache, Duration::from_millis(time)),
                "EXAT" => {
                    let now = SystemTime::now();
                    let now_unix = now
                        .duration_since(UNIX_EPOCH)
                        .expect("Impossibru!")
                        .as_secs();
                    handle_set_with_expiration(args, cache, Duration::from_secs(time - now_unix))
                }
                "PXAT" => {
                    let now = SystemTime::now();
                    let now_unix = now
                        .duration_since(UNIX_EPOCH)
                        .expect("Impossibru!")
                        .as_millis() as u64;
                    handle_set_with_expiration(args, cache, Duration::from_millis(time - now_unix))
                }
                _ => serialize_error("-unknown SET variant"),
            }
        } else {
            serialize_error("-invalid SET expiration")
        }
    } else {
        handle_set_without_expiration(args, cache)
    }
}

fn handle_set_without_expiration(args: &[String], cache: &Cache) -> String {
    if let (Some(key), Some(value)) = (args.get(0), args.get(1)) {
        cache.set(key.clone(), value.clone());
        serialize(InputVariants::StringVariant("+OK".to_string()))
    } else {
        serialize_error("-invalid SET arguments")
    }
}

fn handle_set_with_expiration(args: &[String], cache: &Cache, time: Duration) -> String {
    if let (Some(key), Some(value)) = (args.get(0), args.get(1)) {
        cache.set_with_expiration(key.clone(), value.clone(), time);
        serialize(InputVariants::StringVariant("+OK".to_string()))
    } else {
        serialize_error("-invalid SET arguments")
    }
}

pub fn handle_exists(args: &[String], cache: &Cache) -> String {
    let count = args.iter().filter(|key| cache.exists(key)).count();
    match i32::try_from(count) {
        Ok(count_i32) => serialize(InputVariants::NumberVariant(count_i32)),
        Err(_) => serialize_error("-something went wrong during exists"),
    }
}

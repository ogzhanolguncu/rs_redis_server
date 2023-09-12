use std::{
    borrow::Cow,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    resp::serialize::{serialize, InputVariants},
    store::db::Cache,
};

use super::utils::{serialize_error, throw_err_if_num_of_args_wrong};

pub fn handle_echo(args: &[String]) -> Cow<'static, str> {
    match args.len() {
        1 => serialize(InputVariants::StringVariant(args[0].clone())),
        _ => throw_err_if_num_of_args_wrong("echo"),
    }
}

pub fn handle_ping() -> Cow<'static, str> {
    serialize(InputVariants::StringVariant("pong".to_string()))
}

pub fn ignore_command() -> Cow<'static, str> {
    serialize(InputVariants::Nullish)
}

pub fn handle_get(args: &[String], cache: &Cache) -> Cow<'static, str> {
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

/// args.get(2) and args.get(3) suppose to give us expiration variant such as EX,PX and time of expiration respectively.
pub fn handle_set(args: &[String], cache: &Cache) -> Cow<'static, str> {
    match (args.get(2), args.get(3)) {
        (Some(exp_variant), Some(exp_time_str)) => {
            if let Ok(exp_time) = exp_time_str.parse::<u64>() {
                match exp_variant.as_str() {
                    "EX" => handle_set_with_expiration(args, cache, Duration::from_secs(exp_time)),
                    "PX" => {
                        handle_set_with_expiration(args, cache, Duration::from_millis(exp_time))
                    }
                    "EXAT" => {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Impossibru!");
                        handle_set_with_expiration(
                            args,
                            cache,
                            Duration::from_secs(exp_time - now.as_secs()),
                        )
                    }
                    "PXAT" => {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Impossibru!");
                        handle_set_with_expiration(
                            args,
                            cache,
                            Duration::from_millis(exp_time - now.as_millis() as u64),
                        )
                    }
                    _ => serialize_error("-unknown SET variant"),
                }
            } else {
                serialize_error("-invalid SET expiration")
            }
        }
        _ => handle_set_without_expiration(args, cache),
    }
}

fn handle_set_without_expiration(args: &[String], cache: &Cache) -> Cow<'static, str> {
    if let (Some(key), Some(value)) = (args.get(0), args.get(1)) {
        cache.set(key.clone(), value.clone());
        serialize(InputVariants::StringVariant("+OK".to_string()))
    } else {
        serialize_error("-invalid SET arguments")
    }
}

fn handle_set_with_expiration(args: &[String], cache: &Cache, time: Duration) -> Cow<'static, str> {
    if let (Some(key), Some(value)) = (args.get(0), args.get(1)) {
        cache.set_with_expiration(key.clone(), value.clone(), time);
        serialize(InputVariants::StringVariant("+OK".to_string()))
    } else {
        serialize_error("-invalid SET arguments")
    }
}

pub fn handle_exists(args: &[String], cache: &Cache) -> Cow<'static, str> {
    let count = args.iter().filter(|key| cache.exists(key)).count();
    match i32::try_from(count) {
        Ok(count_i32) => serialize(InputVariants::NumberVariant(count_i32)),
        Err(_) => serialize_error("-something went wrong during exists"),
    }
}

pub fn handle_del(args: &[String], cache: &Cache) -> Cow<'static, str> {
    let count = args.iter().filter(|key| cache.del(key).is_some()).count();
    match i32::try_from(count) {
        Ok(count_i32) => serialize(InputVariants::NumberVariant(count_i32)),
        Err(_) => serialize_error("-something went wrong during del"),
    }
}

pub fn handle_incr(args: &[String], cache: &Cache) -> Cow<'static, str> {
    if let Some(key) = args.get(0) {
        let existing_value = cache.get(key);

        if let Some(value_in_cache) = existing_value {
            match value_in_cache.parse::<i32>().map(|v| v + 1) {
                Ok(new_value) => {
                    cache.set(key.clone(), new_value.to_string());
                    serialize(InputVariants::NumberVariant(new_value))
                }
                Err(_) => serialize_error("-could not parse stored number"),
            }
        } else {
            cache.set(key.clone(), 1.to_string());
            serialize(InputVariants::NumberVariant(1))
        }
    } else {
        serialize_error("-invalid INCR arguments")
    }
}

pub fn handle_decr(args: &[String], cache: &Cache) -> Cow<'static, str> {
    if let Some(key) = args.get(0) {
        let existing_value = cache.get(key);

        if let Some(value_in_cache) = existing_value {
            match value_in_cache.parse::<i32>().map(|v| v - 1) {
                Ok(new_value) => {
                    cache.set(key.clone(), new_value.to_string());
                    serialize(InputVariants::NumberVariant(new_value))
                }
                Err(_) => serialize_error("-could not parse stored number"),
            }
        } else {
            cache.set(key.clone(), (-1).to_string());
            serialize(InputVariants::NumberVariant(-1))
        }
    } else {
        serialize_error("-invalid INCR arguments")
    }
}

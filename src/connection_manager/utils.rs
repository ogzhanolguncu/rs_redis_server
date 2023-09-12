use std::borrow::Cow;

use crate::resp::serialize::{serialize, InputVariants};

pub fn throw_err_if_num_of_args_wrong(variant: &str) -> Cow<'static, str> {
    serialize(InputVariants::StringVariant(concat_string!(
        "-ERR wrong number of arguments for ",
        variant,
        " command"
    )))
}

pub fn serialize_error(message: &str) -> Cow<'static, str> {
    println!("{}", message);
    serialize(InputVariants::ErrorVariant(message.to_string()))
}

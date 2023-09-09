use crate::resp::serialize::{serialize, InputVariants};

pub fn err_if_num_of_args_wrong(variant: &str) -> String {
    return serialize(InputVariants::StringVariant(concat_string!("-ERR wrong number of arguments for ",variant," command")));
}
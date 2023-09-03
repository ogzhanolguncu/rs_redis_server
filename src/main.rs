use resp::deserialize::parse_resp;

mod resp;

#[macro_use(concat_string)]
extern crate concat_string;

fn main() {
    match parse_resp("*2\r\n$7\r\nCOMMAND\r\n$4\r\nDOCS\r\n") {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

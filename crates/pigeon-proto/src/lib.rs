pub mod errors;

use prost::Message;

pub mod test {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

pub mod main {
    include!(concat!(env!("OUT_DIR"), "/main.rs"));
}

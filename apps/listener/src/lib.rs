pub mod kafka;
pub mod parser;
pub mod rpc;
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/larptech.rs"));
}

use std::collections::HashMap;

use data_encoding::BASE64URL_NOPAD;
use lazy_static::lazy_static;
use openssl::sha::sha256;

pub const VALIDATOR_AS_BUFFER: &'static [u8] = "Validator".as_bytes();
pub const BUNDLR_AS_BUFFER: &[u8] = "Bundlr".as_bytes();

lazy_static! {
    pub static ref VALIDATOR_ADDRESS: String = {
        let key = serde_json::from_slice::<HashMap<String, String>>(include_bytes!("../wallet.json")).unwrap();

        BASE64URL_NOPAD.encode(&sha256(&BASE64URL_NOPAD.decode(key.get("n").unwrap().as_bytes()).unwrap()))
    };
}
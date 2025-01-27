#![no_main]
use candid::{define_function, CandidType, Decode, Deserialize, Nat};
use libfuzzer_sys::fuzz_target;
use serde_bytes::ByteBuf;

#[derive(CandidType, Deserialize)]
pub struct Token {
    pub key: String,
    pub content_encoding: String,
    pub index: Nat,
    pub sha256: Option<ByteBuf>,
}

define_function!(pub callback : (&u8) -> (Nat));
#[derive(CandidType, Deserialize)]
pub struct CallbackStrategy {
    pub callback: callback,
    pub token: Token,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback(CallbackStrategy),
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub streaming_strategy: Option<StreamingStrategy>,
}

fuzz_target!(|data: &[u8]| {
    let payload = data.to_vec();
    let _decoded = match Decode!(payload.as_slice(), HttpResponse) {
        Ok(_v) => _v,
        Err(_e) => return,
    };
});

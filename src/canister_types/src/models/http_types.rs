use candid::CandidType;
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

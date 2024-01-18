use std::fmt;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, Eq, Hash)]
pub struct Identifier {
    id: u64,
    principal: Principal,
    kind: String,
}

impl Identifier {
    pub fn new(id: u64, principal: Principal, kind: String) -> Result<Identifier, String> {
        if kind.len() != 3 {
            return Err("New - Invalid identifier: 'kind' length needs to be 3".to_string());
        }

        Ok(Identifier {
            id,
            principal,
            kind,
        })
    }

    pub fn encode(&self) -> Result<Principal, String> {
        if self.kind.len() != 3 {
            return Err("Encode - Invalid identifier: 'kind' length needs to be 3".to_string());
        }
        let mut array = Vec::new();
        array.extend_from_slice(b"\x0Acat");
        array.extend_from_slice(&self.kind.as_bytes().to_vec());
        array.extend_from_slice(&self.principal.as_slice());
        array.extend_from_slice(&Self::to_u32_be_bytes(self.id));
        Ok(Principal::from_slice(&array))
    }

    pub fn decode(encoded_identifier: &Principal) -> (u64, Principal, String) {
        let mut p = encoded_identifier.as_slice().to_vec();
        let custom_identifier = p[..4].to_vec();

        if custom_identifier == b"\x0Acat".to_vec() {
            p.drain(..4);
            let kind = String::from_utf8(p[..3].to_vec()).unwrap();
            p.drain(..3);

            let index_bytes = p.drain(p.len() - 4..).collect::<Vec<u8>>();
            let index = Self::from32bits(&index_bytes);
            return (index, Principal::from_slice(&p), kind);
        } else {
            return (0, *encoded_identifier, "principal".to_string());
        }
    }

    pub fn id(encoded_identifier: &Principal) -> u64 {
        Self::decode(encoded_identifier).0
    }

    pub fn principal(encoded_identifier: &Principal) -> Principal {
        Self::decode(encoded_identifier).1
    }

    pub fn kind(encoded_identifier: &Principal) -> String {
        Self::decode(encoded_identifier).2
    }

    fn to_u32_be_bytes(n: u64) -> [u8; 4] {
        [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8]
    }

    fn from32bits(ba: &[u8]) -> u64 {
        let mut value = 0;
        for i in 0..4 {
            value = (value << 8) | (ba[i] as u64);
        }
        value
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {} - canister: {} - kind: {}",
            self.id.to_string(),
            self.principal.to_string(),
            self.kind,
        )
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.encode()
            .map_or(false, |s| Self::encode(other).map_or(false, |o| s == o))
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            id: Default::default(),
            principal: Principal::anonymous(),
            kind: Default::default(),
        }
    }
}

#[test]
fn test_decode() {
    let x = Identifier::new(
        0,
        Principal::from_text("ve3v4-o7xuv-ijejl-vcyfx-hjy3b-owwtx-jte2k-2bciw-spskd-jgmvd-rqe")
            .unwrap(),
        "prl".to_string(),
    )
    .unwrap()
    .encode();
    match x {
        Ok(_id) => println!("{:?}", _id.to_string()),
        Err(_err) => println!("{:?}", _err),
    }
    // let (id, principal, kind) = Identifier::decode(
    //     &Principal::from_text("ve3v4-o7xuv-ijejl-vcyfx-hjy3b-owwtx-jte2k-2bciw-spskd-jgmvd-rqe")
    //         .unwrap(),
    // );
    // println!(
    //     "id: {}\nprincipal: {}\nkind: {}",
    //     id,
    //     principal.to_string(),
    //     kind
    // );
}

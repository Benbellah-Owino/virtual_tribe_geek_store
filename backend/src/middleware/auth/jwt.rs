use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub email: String,
    pub username: String,
    pub acc_type: String,
    pub verified: bool,
    pub exp: usize,
}

impl fmt::Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n id: {},\n email: {},\n username: {},\n acc_type: {},\n verified: {},\n}} ",
            self.id, self.email, self.username, self.acc_type, self.verified
        )
    }
}

pub fn gen_token(claims: &Claims, secret: &str) -> Result<String, Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    println!("\n------------------------------------------------------------------------------\n\n{token}");
    let c = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    match c {
        Ok(tk) => Ok(tk.claims),
        Err(e) => {
            Err(e)
        }
    }
}

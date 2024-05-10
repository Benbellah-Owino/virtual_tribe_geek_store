use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub id: String,
    pub email: String,
    pub username: String,
    pub acc_type : String,
    pub verified: bool,
    pub exp: usize
}

pub fn gen_cookie(claims: &Claims) -> Result<String, Error>{
    encode(&Header::default(), claims, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn decode_cookie(token: &str){
    println!("\n--------------------------------------------------\n\n{token}");
    let c = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());

    match c{
        Ok(tk) =>{
            println!("{:?}", tk);
        },
        Err(e) =>{
            dbg!(e);
        }
    }
}
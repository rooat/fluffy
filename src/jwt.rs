use jsonwebtoken::{TokenData, Validation, Header};
use serde_derive::{Deserialize, Serialize};
//use serde::de::DeserializeWoned;
use serde::{de, ser};
use crate::{datetime};

/// secret key
const KEY: [u8; 16] = *include_bytes!("./secret.key");

/// 1天
pub const DAY_ONE: u64 = 86400;

/// 用户token
#[derive(Serialize, Deserialize)]
pub struct UserToken { 
    pub exp: u64, //过期时间
    pub id: usize, //用户编号
    pub name: String, //用户名称
    //pub token: String, //token
}

/// 生成token
pub fn encode(id: usize, name: &String) -> String { 
    let now = datetime::timestamp();
    //let token = random::rand_str(32);
    let playload = UserToken { 
        exp: now + DAY_ONE, 
        id,
        name: name.to_owned(),  //token,
    };
    jsonwebtoken::encode(&Header::default(), &playload, &KEY).unwrap()
}

    /// 生成token - 自定义格式
    pub fn encode_by<T: ser::Serialize>(data: &T) -> String { 
    //let token = random::rand_str(32);
    jsonwebtoken::encode::<T>(&Header::default(), &data, &KEY).unwrap()
}

/// 解码authorization字段 - 默认方式
pub fn decode(token: &str) 
    -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(token, &KEY, &Validation::default())
}

/// 解码authorization字段 - 自定义方式
pub fn decode_by<T: de::DeserializeOwned>(token: &str) 
    -> jsonwebtoken::errors::Result<TokenData<T>> {
    jsonwebtoken::decode::<T>(token, &KEY, &Validation::default())
}

/// 校验是否正确
pub fn verify<T: ser::Serialize>(_token_data: &TokenData<T>) -> Result<String, String> { 
    Ok("成功".to_owned())
}

use redis::{Client, Connection};
use lazy_static::*;
use std::sync::{Mutex};

lazy_static! {
    static ref CACHES: Mutex<Vec<Client>> = Mutex::new(vec![]);
}

/// 初始化資料庫連接
///
/// 連接字符串形似: 'redis://127.0.0.1'
pub fn init_connections(conn_string: &str) { 
    println!("建立缓存连接: {}", conn_string);
    let cache = redis::Client::open(conn_string).unwrap();
    let mut pools = CACHES.lock().unwrap();
    (*pools).push(cache);
}

/// 獲取資料庫連接
///
/// ```rust
/// let mut redis = cache::get_conn();
/// let _val = redis.get::<&str, String>("hello").unwrap_or("world".to_owned());
/// ```
pub fn get_conn() -> Connection { 
    let pools = CACHES.lock().unwrap();
    unsafe { 
        (*pools).get_unchecked(0).get_connection().unwrap()
    }
}

/// 用於生成密鑰
pub const KEY: [u8; 16] = *include_bytes!("../secret.key");

/// authorization 的名称
///
/// 一般情況下格式
///
/// Authorization: Bearer xxxxxxxxx
pub const AUTHORIZATION: &'static str = "Authorization";

/// 秒數：一周
pub const ONE_WEEK: u64 = 86400 * 7;

/// 秒数: 一天
/// 
/// 一般用於設置過期時間
pub const ONE_DAY: u64 = 86400;


#[allow(dead_code)]
pub struct Email(String);

impl Email {
    /// # Description
    ///     Email 值对象，格式验证逻辑
    /// # Param
    ///     email: String, 需要验证的 email 值
    /// # Return
    ///     Self: 初始化后的应用层实例
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains("@") {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }
}
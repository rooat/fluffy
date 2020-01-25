use std::default::Default;

#[derive(Default)]
pub struct Validator {
    errors: Vec<String>,
}

impl Validator {

    /// 生成校驗器
    pub fn new() -> Self { 
        Self { 
            errors: vec![],
        }       
    }

    /// 用户名稱
    /// 需要6-15位, a-zA-Z0-9, 且不能以数字开头
    pub fn is_username<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 15 {
            self.errors.push(message.to_owned());
        }
        self
    }

    /// 是否密碼
    pub fn is_password<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 20 { 
            self.errors.push(message.to_owned());
        }
        // TODO: need to check the illegal letters
        self
    }

    /// 是否是重复密碼
    pub fn is_re_password<T: AsRef<str>>(&mut self, value: &T, re_value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 20 { 
            self.errors.push(message.to_owned());
        }
        let length2 = re_value.as_ref().chars().count();
        if length2 < 6 ||  length2 > 20 { 
            self.errors.push(message.to_owned());
        }
        if value.as_ref() != re_value.as_ref() { 
            self.errors.push(message.to_owned());
        }
        // TODO: need to check the illegal letters
        self
    }

    /// 是否是昵稱
    pub fn is_nickname<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self { 
        let length = value.as_ref().chars().count();
        if length < 3 || length > 12 {
            self.errors.push(message.to_owned());
        }
        self
    }

    /// 是否是驗證碼
    pub fn is_code(&mut self, value: u32, message: &str) -> &mut Self { 
        if value < 10000 || value > 99999 { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// is status
    pub fn is_state(&mut self, value: u32, message: &str) -> &mut Self { 
        if value != 0 && value != 1 { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// in range ?
    pub fn in_range(&mut self, min: u32, max: u32, value: u32, message: &str) -> &mut Self { 
        if min < value || max > value { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// get validated result
    pub fn validate(&mut self) -> Result<(), String> {
        if self.errors.len() > 0 {
            return Err(self.errors.join(","));
        }
        Ok(())
    }
}

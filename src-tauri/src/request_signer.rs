const GMT_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT+00:00";
const KEY: &str = "526EBA802E6FCF44661DE4393A82ABDA";

fn get_time() -> String {
    Utc::now().format(GMT_FORMAT).to_string()
}

use chrono::Utc;
use indexmap::IndexMap;

/// 表单类型枚举
pub enum FormData<'a> {
    /// 无表单（普通GET请求等）
    None,
    /// 登录表单 - 保留原始键值对
    Login(&'a IndexMap<&'a str, &'a str>),
    /// 普通表单 - 只保留签名摘要
    Normal(&'a IndexMap<&'a str, &'a str>),
}

/// 请求上下文
pub struct RequestContext<'a> {
    pub url: &'a str,
    pub form: FormData<'a>,
    pub user_id: Option<&'a str>,
    pub access_secret: Option<&'a str>,
}

impl<'a> RequestContext<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            form: FormData::None,
            user_id: None,
            access_secret: None,
        }
    }

    pub fn with_form(mut self, form: &'a IndexMap<&'a str, &'a str>, is_login: bool) -> Self {
        self.form = if is_login {
            FormData::Login(form)
        } else {
            FormData::Normal(form)
        };
        self
    }

    pub fn with_user_id(mut self, user_id: &'a str ) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_secret(mut self, secret: &'a str) -> Self {
        self.access_secret = Some(secret);
        self
    }
}

fn form_sign(form_str: &str) -> String {
    let digest = md5::compute(form_str.as_bytes());
    format!("{:x}", digest) // 返回32位十六进制字符串
}
// HMAC-SHA1签名
fn make_digest(message: &str, key: &str) -> String {
    let digest = hmac_sha1::hmac_sha1(key.as_bytes(), message.as_bytes());
    hex::encode(digest)
    // 返回32位十六进制字符串
}


// 拼接字符串
fn hash_string(
    url: &str,
    time_str: &str,
    form: &FormData,
    user_id: Option<&str>,
) -> String {
    // 第一层：user_id
    let result = match user_id {
        Some(uid) => format!("{}|{}|{}", url, uid.to_uppercase(), time_str),
        None => format!("{}|{}", url, time_str),
    };

    // 第二层：表单处理
    match form {
        FormData::None => result,
        FormData::Login(form_data) => {
            let form_str = form_data
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("|");
            format!("{}|{}", result, form_str)
        }
        FormData::Normal(form_data) => {
            let form_str = form_data
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("|");
            format!("{}|{}", result, form_sign(&form_str).to_uppercase())
        }
    }
}

// 签名请求->返回 String 的时间和签名
pub fn sign_request(ctx: &RequestContext) -> (String, String) {
    let time_str = get_time();
    let string = hash_string(
        &ctx.url,
        &time_str,
        &ctx.form,
        ctx.user_id.as_deref(),
    );
    let secret = ctx.access_secret.as_deref().unwrap_or(KEY);
    let signature = make_digest(&string, secret);
    (time_str, signature)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_request_returns_sha1_hex_signature() {
        let mut form = IndexMap::new();
        form.insert("account_name", "demo-user");
        form.insert("app_id", "MTANDROID");
        form.insert("app_version_name", "5.1.1");
        form.insert("app_version_number", "111");
        form.insert("device_type", "ANDROID");
        form.insert("dpr", "2.7");
        form.insert("system_version", "8.1.0");
        form.insert("user_pwd", "demo-password");

        let (time_str, sign_str) = sign_request(
            &RequestContext::new("https://api.mosoteach.cn/mssvc/index.php/passport/login")
                .with_form(&form, true),
        );

        assert!(time_str.contains("GMT+00:00"));
        assert_eq!(sign_str.len(), 40);
        assert!(sign_str.chars().all(|ch| ch.is_ascii_hexdigit()));
    }
}

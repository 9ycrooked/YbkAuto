// use reqwest::Client;
// use crate::request_signer;
// use crate::api::URL_CC_LIST_JOINED;
// use crate::login::{base_headers, LoginResponse};
//
// pub async fn list_course(login_response: LoginResponse) -> Result<(), String> {
//     let url_list_joined = URL_CC_LIST_JOINED;
//     let (time, sign) = request_signer::sign_request(url_list_joined, None, None, None, true);
//     let client = Client::builder()
//         .default_headers(base_headers())
//         .build()
//         .map_err(|e| format!("创建客户端失败: {}", e))?;
//     let response = client
//         .post(url_list_joined)
//         // 只需追加签名相关的 2 个头
//         .header("Date", &time)
//         .header("X-mssvc-access-id", &login_response.user.access_id)
//         .header("X-mssvc-signature", &sign)
//         .header("X-mssvc-sec-ts", &login_response.user.access_secret)
//         .send()
//         .await
//         .map_err(|e| format!("发送请求失败: {}", e))?;
//     response
//         .text()
//         .await
//         .map_err(|e| format!("解析响应失败: {}", e))?;
//     Ok(())
// }
// #[cfg(test)]
// mod tests {
//     use crate::login::LoginResponse;
//     use super::list_course;
//
//     #[tokio::test]
//     async fn test_list_course() {
//         let class = list_course();
//     }
// }

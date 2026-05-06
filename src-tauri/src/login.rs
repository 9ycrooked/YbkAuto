use crate::api::{
    URL_CC_LIST_JOINED, URL_CC_RESOURCE_DOWNLOAD, URL_CC_RESOURCE_LIST, URL_CC_RESOURCE_RECORDS,
    URL_CC_RESOURCE_VIEWER, URL_LOGIN,
};
use futures::future::join_all;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Manager};
use tokio::sync::Semaphore;

const SESSION_FILE_NAME: &str = "session.json";

#[derive(Clone)]
pub struct MosoteachClient {
    http_client: Arc<Client>,
    user: Option<User>,
    user_id: Option<String>,
    token: Option<String>,
}

pub fn web_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Origin",
        HeaderValue::from_static("https://www.mosoteach.cn"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://www.mosoteach.cn/"),
    );
    headers.insert("x-client-app-id", HeaderValue::from_static("MTWEB"));
    headers.insert("x-client-version", HeaderValue::from_static("6.0.0"));
    headers.insert(
        "x-security-type",
        HeaderValue::from_static("SECURITY_TYPE_TOKEN"),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct LoginApiResponse {
    pub user: User,
    pub token: String,
    pub status: bool,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct User {
    #[serde(alias = "userId")]
    pub user_id: String,
    #[serde(alias = "accessId")]
    pub access_id: String,
    #[serde(alias = "accessSecret")]
    pub access_secret: String,
    #[serde(alias = "fullName")]
    pub full_name: String,
    #[serde(alias = "nickName")]
    pub nick_name: Option<String>,
    #[serde(alias = "phoneNumber")]
    pub phone_number: String,
    #[serde(alias = "studentNo")]
    pub student_no: Option<String>,
    #[serde(alias = "bindSchool")]
    pub bind_school: Option<BindSchool>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BindSchool {
    #[serde(alias = "schoolId")]
    pub school_id: Option<String>,
    #[serde(alias = "schoolName")]
    pub school_name: Option<String>,
    #[serde(alias = "departmentId")]
    pub department_id: Option<String>,
    #[serde(alias = "departmentName")]
    pub department_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct CourseListApiResponse {
    #[serde(alias = "clazzCourses")]
    pub clazz_courses: Vec<ClazzCourse>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClazzCourse {
    #[serde(alias = "id", default)]
    pub id: String,
    pub course: CourseInfo,
    pub clazz: ClazzInfo,
    #[serde(alias = "fullCoverUrl", default)]
    pub full_cover_url: Option<String>,
    #[serde(alias = "creater", default)]
    pub creater: Option<CourseCreater>,
    #[serde(alias = "createTime", default)]
    pub create_time: Option<String>,
    #[serde(alias = "status", default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CourseInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClazzInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CourseCreater {
    #[serde(alias = "fullName", default)]
    pub full_name: String,
    #[serde(alias = "userId", default)]
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
struct ResourceListResponse {
    resources: Vec<ResourceItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResourceItem {
    #[serde(alias = "id")]
    _id: String,
    #[serde(alias = "score")]
    score: Option<f64>,
    #[serde(alias = "obtainScore")]
    obtain_score: Option<f64>,
    #[serde(alias = "fullCoverUrl", default)]
    full_cover_url: Option<String>,
    #[serde(alias = "viewFlag", default)]
    _view_flag: Option<String>,
    #[serde(alias = "viewCount", default)]
    _view_count: Option<i32>,
    #[serde(alias = "mimeType", default)]
    mime_type: Option<String>,
    #[serde(alias = "metaDuration", default)]
    meta_duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ResourceRecordResponse {
    record: ResourceRecord,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ResourceRecord {
    #[serde(alias = "watchTo", default)]
    _watch_to: i32,
    #[serde(alias = "lastWatchTo", default)]
    _last_watch_to: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ViewerResponse {
    #[serde(alias = "url", default)]
    url: Option<String>,
    #[serde(alias = "cover", default)]
    _cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResult {
    pub total: usize,
    pub completed: usize,
    pub failed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceState {
    pub completed: usize,
    pub incomplete: usize,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self {
            completed: 0,
            incomplete: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseSummary {
    #[serde(alias = "clazzCourseId", alias = "clazz_course_id")]
    pub clazz_course_id: String,
    #[serde(alias = "courseName", alias = "course_name")]
    pub course_name: String,
    #[serde(alias = "className", alias = "class_name")]
    pub class_name: Option<String>,
    #[serde(alias = "teacherName", alias = "teacher_name")]
    pub teacher_name: String,
    #[serde(alias = "courseStatus", alias = "course_status")]
    pub course_status: String,
    #[serde(alias = "createTime", alias = "create_time")]
    pub create_time: Option<String>,
    pub resource_state: ResourceState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardState {
    pub courses: Vec<CourseSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub user_id: String,
    pub full_name: String,
    pub school_name: Option<String>,
    pub student_no: Option<String>,
    pub department_name: Option<String>,
}

impl From<&User> for SessionUser {
    fn from(user: &User) -> Self {
        let school_name = user
            .bind_school
            .as_ref()
            .and_then(|s| s.school_name.clone());
        let department_name = user
            .bind_school
            .as_ref()
            .and_then(|s| s.department_name.clone());
        Self {
            user_id: user.user_id.clone(),
            full_name: user.full_name.clone(),
            school_name,
            student_no: user.student_no.clone(),
            department_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    pub authenticated: bool,
    pub remembered_username: String,
    pub user: Option<SessionUser>,
    pub dashboard: Option<DashboardState>,
}

impl SessionState {
    fn unauthenticated(remembered_username: String) -> Self {
        Self {
            authenticated: false,
            remembered_username,
            user: None,
            dashboard: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct StoredSession {
    #[serde(default)]
    username: String,
    #[serde(default)]
    user: Option<SessionUser>,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(default)]
    token: Option<String>,
}

impl StoredSession {
    fn username_only(username: String) -> Self {
        Self {
            username,
            ..Default::default()
        }
    }

    fn has_token(&self) -> bool {
        self.user_id.is_some() && self.token.is_some()
    }
}

impl MosoteachClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .default_headers(web_headers())
            .build()
            .expect("无法创建 HTTP 客户端");

        Self {
            http_client: Arc::new(http_client),
            user: None,
            user_id: None,
            token: None,
        }
    }

    fn restore_session(&mut self, session: &StoredSession) {
        self.user_id = session.user_id.clone();
        self.token = session.token.clone();
    }

    fn to_stored_session(&self, username: String) -> StoredSession {
        StoredSession {
            username,
            user: self.user.as_ref().map(SessionUser::from),
            user_id: self.user_id.clone(),
            token: self.token.clone(),
        }
    }

    pub async fn login(
        &mut self,
        username: &str,
        ciphertext: &str,
    ) -> Result<(User, String), String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!("{}?_ts={}", URL_LOGIN, timestamp);

        let body = serde_json::json!({
            "account": username,
            "ciphertext": ciphertext
        });

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|error| format!("网络请求失败: {}", error))?;

        let body_text = response
            .text()
            .await
            .map_err(|e| format!("读取响应失败: {}", e))?;

        if !body_text.starts_with('{') {
            return Err(format!("服务器返回非JSON响应: {}", body_text));
        }

        let login_response: LoginApiResponse =
            serde_json::from_str(&body_text).map_err(|e| format!("JSON解析失败: {}", e))?;

        if !login_response.status {
            return Err(format!("登录失败"));
        }

        self.user = Some(login_response.user.clone());
        self.user_id = Some(login_response.user.user_id.clone());
        self.token = Some(login_response.token.clone());

        Ok((login_response.user, login_response.token))
    }

    pub async fn list_course(&self) -> Result<Vec<ClazzCourse>, String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!("{}?_ts={}", URL_CC_LIST_JOINED, timestamp);

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|error| format!("获取课程列表失败: {}", error))?;

        let body_text = response
            .text()
            .await
            .map_err(|e| format!("读取课程响应失败: {}", e))?;

        let course_response: CourseListApiResponse = serde_json::from_str(&body_text)
            .map_err(|error| format!("课程JSON解析失败: {}", error))?;

        Ok(course_response.clazz_courses)
    }

    pub async fn list_resources(&self, ccid: &str) -> Result<Vec<ResourceItem>, String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        // 避免高频请求，等待一下
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let url = format!(
            "{}/{}/resources?roleId=2&_ts={}",
            URL_CC_RESOURCE_LIST, ccid, timestamp
        );

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|e| format!("获取资源列表失败: {}", e))?;

        let body_text = response
            .text()
            .await
            .map_err(|e| format!("读取资源响应失败: {}", e))?;

        let result: ResourceListResponse =
            serde_json::from_str(&body_text).map_err(|e| format!("资源JSON解析失败: {}", e))?;

        Ok(result.resources)
    }

    pub async fn view_resource(&self, ccid: &str, resource_id: &str) -> Result<(), String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!(
            "{}/{}/resources/{}/viewer?roleId=2&_ts={}",
            URL_CC_RESOURCE_VIEWER, ccid, resource_id, timestamp
        );

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|e| format!("访问资源失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("访问资源失败:状态码{}", response.status()));
        }

        Ok(())
    }

    pub async fn get_viewer_url(
        &self,
        ccid: &str,
        resource_id: &str,
    ) -> Result<ViewerResponse, String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!(
            "{}/{}/resources/{}/viewer?roleId=2&_ts={}",
            URL_CC_RESOURCE_VIEWER, ccid, resource_id, timestamp
        );

        println!("[Complete] 请求viewer API: {}", url);

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|e| format!("访问资源失败: {}", e))?;

        println!("[Complete] viewer API状态: {}", response.status());

        if !response.status().is_success() {
            return Err(format!("访问资源失败:状态码{}", response.status()));
        }

        let body_text = response
            .text()
            .await
            .map_err(|e| format!("读取响应失败: {}", e))?;
        println!("[Complete] viewer API响应: {}", body_text);

        let viewer_resp: ViewerResponse =
            serde_json::from_str(&body_text).map_err(|e| format!("解析viewer响应失败: {}", e))?;

        println!("[Complete] viewer url字段: {:?}", viewer_resp.url);

        Ok(viewer_resp)
    }

    pub async fn fetch_m3u8(&self, url: &str) -> Result<String, String> {
        let response = self
            .http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| format!("获取m3u8失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("获取m3u8失败:状态码{}", response.status()));
        }

        response
            .text()
            .await
            .map_err(|e| format!("读取m3u8内容失败: {}", e))
    }

    pub async fn fetch_ts_segments(&self, base_url: &str, count: usize) -> Result<(), String> {
        let sem = Arc::new(Semaphore::new(10));
        let mut handles = Vec::new();

        for i in 0..count {
            let url = format!("{}{:04}.ts", base_url, i);
            let sem = sem.clone();
            let http = self.http_client.clone();

            handles.push(async move {
                let _permit = sem.acquire().await.unwrap();
                match http
                    .get(&url)
                    .timeout(std::time::Duration::from_secs(15))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            println!("[Complete] 分片请求成功: segment={}", i);
                        }
                    }
                    Err(e) => {
                        println!("[Complete] 分片请求失败: segment={}, error={}", i, e);
                    }
                }
            });
        }

        join_all(handles).await;
        Ok(())
    }

    pub async fn download_resource_api(&self, ccid: &str, resource_id: &str) -> Result<(), String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!(
            "{}/{}/resources/{}/download?_ts={}",
            URL_CC_RESOURCE_DOWNLOAD, ccid, resource_id, timestamp
        );

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|e| format!("下载资源API调用失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("下载资源API失败:状态码{}", response.status()));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_resource_records(
        &self,
        ccid: &str,
        resource_id: &str,
    ) -> Result<ResourceRecord, String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!(
            "{}/{}/resources/{}/records?_ts={}",
            URL_CC_RESOURCE_RECORDS, ccid, resource_id, timestamp
        );

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|e| format!("获取资源记录失败: {}", e))?;

        let body_text = response
            .text()
            .await
            .map_err(|e| format!("读取资源记录响应失败: {}", e))?;

        let result: ResourceRecordResponse =
            serde_json::from_str(&body_text).map_err(|e| format!("资源记录JSON解析失败: {}", e))?;

        Ok(result.record)
    }

    pub async fn update_watch_progress(
        &self,
        ccid: &str,
        resource_id: &str,
        watch_to: i32,
        duration: f64,
    ) -> Result<(), String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let url = format!(
            "{}/{}/resources/{}/records?_ts={}",
            URL_CC_RESOURCE_RECORDS, ccid, resource_id, timestamp
        );

        // 完整的进度上报，包含 currentWatchTo 和 duration
        // 当 currentWatchTo >= duration 时，服务端自动标记完成
        let body = serde_json::json!({
            "watchTo": watch_to,
            "currentWatchTo": duration,
            "duration": duration
        });

        println!(
            "[Complete] 进度上报: watchTo={}, currentWatchTo={}, duration={}",
            watch_to, duration, duration
        );

        let response = self
            .http_client
            .post(&url)
            .header("x-token", token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("更新观看进度失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("更新观看进度失败:状态码{}", response.status()));
        }

        Ok(())
    }

    // 发送多次进度上报确保完成信号被可靠送达
    pub async fn report_progress_multiple(
        &self,
        ccid: &str,
        resource_id: &str,
        watch_to: i32,
        duration: f64,
        times: usize,
    ) -> Result<(), String> {
        for i in 0..times {
            println!(
                "[Complete] 进度上报 {}/{}: watchTo={}, currentWatchTo={}",
                i + 1,
                times,
                watch_to,
                duration
            );
            if let Err(e) = self
                .update_watch_progress(ccid, resource_id, watch_to, duration)
                .await
            {
                println!("[Complete] 第{}次上报失败: {}", i + 1, e);
            }
            // 间隔一小段时间
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn download_resource(&self, url: &str) -> Result<Option<String>, String> {
        let token = self
            .token
            .as_deref()
            .ok_or_else(|| "token 未登录".to_string())?;

        let response = self
            .http_client
            .get(url)
            .header("x-token", token)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Referer", "https://www.mosoteach.cn/")
            .header("Origin", "https://www.mosoteach.cn")
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("下载资源失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("下载资源失败: 状态码{}", response.status()));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("读取下载内容失败: {}", e))?;

        // 获取临时目录
        let temp_dir = std::env::temp_dir();
        let file_name = format!(
            "ybk_resource_{}.tmp",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let file_path = temp_dir.join(file_name);

        std::fs::write(&file_path, &bytes).map_err(|e| format!("保存临时文件失败: {}", e))?;

        Ok(Some(file_path.to_string_lossy().to_string()))
    }
}

pub async fn bootstrap_session(app: &AppHandle) -> Result<SessionState, String> {
    let Some(stored_session) = load_stored_session(app)? else {
        return Ok(SessionState::unauthenticated(String::new()));
    };

    if !stored_session.has_token() {
        return Ok(SessionState::unauthenticated(stored_session.username));
    }

    let remembered_username = stored_session.username.clone();
    let user = stored_session.user.clone();
    let mut client = MosoteachClient::new();
    client.restore_session(&stored_session);

    match build_dashboard(&client).await {
        Ok(dashboard) => Ok(SessionState {
            authenticated: true,
            remembered_username,
            user,
            dashboard: Some(dashboard),
        }),
        Err(_) => {
            save_stored_session(
                app,
                &StoredSession::username_only(remembered_username.clone()),
            )?;
            Ok(SessionState::unauthenticated(remembered_username))
        }
    }
}

pub async fn login_and_build_session(
    app: &AppHandle,
    username: String,
    ciphertext: String,
) -> Result<SessionState, String> {
    let username = username.trim().to_string();

    if username.is_empty() {
        return Err("请输入账号".to_string());
    }

    if ciphertext.is_empty() {
        return Err("加密后的密码不能为空".to_string());
    }

    save_stored_session(app, &StoredSession::username_only(username.clone()))?;

    let mut client = MosoteachClient::new();
    let (user, _token) = client.login(&username, &ciphertext).await?;
    let dashboard = build_dashboard(&client).await?;
    let session_user = SessionUser::from(&user);

    save_stored_session(app, &client.to_stored_session(username.clone()))?;

    Ok(SessionState {
        authenticated: true,
        remembered_username: username,
        user: Some(session_user),
        dashboard: Some(dashboard),
    })
}

pub async fn refresh_dashboard(app: &AppHandle) -> Result<DashboardState, String> {
    let stored_session =
        load_stored_session(app)?.ok_or_else(|| "未找到登录会话，请重新登录".to_string())?;

    if !stored_session.has_token() {
        return Err("当前没有可用的登录令牌，请重新登录".to_string());
    }

    let mut client = MosoteachClient::new();
    client.restore_session(&stored_session);

    build_dashboard(&client).await
}

pub fn logout(app: &AppHandle) -> Result<(), String> {
    let username = load_stored_session(app)?
        .map(|session| session.username)
        .unwrap_or_default();

    save_stored_session(app, &StoredSession::username_only(username))
}

fn session_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    path.push(SESSION_FILE_NAME);
    Ok(path)
}

fn load_stored_session(app: &AppHandle) -> Result<Option<StoredSession>, String> {
    let path = session_file_path(app)?;

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let session =
        serde_json::from_str::<StoredSession>(&content).map_err(|error| error.to_string())?;
    Ok(Some(session))
}

fn save_stored_session(app: &AppHandle, session: &StoredSession) -> Result<(), String> {
    let path = session_file_path(app)?;
    write_json_file(&path, session)
}

fn write_json_file<T: Serialize>(path: &Path, data: &T) -> Result<(), String> {
    let content = serde_json::to_string_pretty(data).map_err(|error| error.to_string())?;
    fs::write(path, content).map_err(|error| error.to_string())
}

fn is_resource_completed(r: &ResourceItem) -> bool {
    let score = r.score.unwrap_or(0.0);
    let obtain = r.obtain_score.unwrap_or(-1.0);
    score > 0.0 && obtain >= score
}

fn clean_cdn_url(url: &str) -> String {
    let mut parts = url.splitn(2, '?');
    let base = parts.next().unwrap_or(url);
    let query = parts.next();

    match query {
        Some(qs) => {
            let filtered: Vec<&str> = qs
                .split('&')
                .filter(|p| !p.starts_with("x-oss-process") && !p.starts_with("x-oss-process="))
                .collect();
            if filtered.is_empty() {
                base.to_string()
            } else {
                format!("{}?{}", base, filtered.join("&"))
            }
        }
        None => base.to_string(),
    }
}

async fn build_dashboard(client: &MosoteachClient) -> Result<DashboardState, String> {
    let courses = client.list_course().await?;
    println!("[Dashboard] 课程数量: {}", courses.len());

    // 按课程ID排序，确保顺序一致
    let mut course_ids: Vec<_> = courses.iter().map(|c| c.id.clone()).collect();
    course_ids.sort();
    println!("[Dashboard] 课程ID列表(排序后): {:?}", course_ids);

    let sem = Arc::new(Semaphore::new(10));

    let futures = courses.iter().map(|course| {
        let sem = sem.clone();
        let client = client.clone();
        let ccid = course.id.clone();
        async move {
            let _permit = sem.acquire().await.unwrap();
            let resources = client.list_resources(&ccid).await.unwrap_or_default();
            let completed = resources
                .iter()
                .filter(|r| is_resource_completed(r))
                .count();
            let incomplete = resources.len() - completed;
            (
                course.id.clone(),
                ResourceState {
                    completed,
                    incomplete,
                },
            )
        }
    });

    let results: HashMap<String, ResourceState> = join_all(futures).await.into_iter().collect();

    let course_summaries = courses
        .into_iter()
        .map(|c| CourseSummary {
            clazz_course_id: c.id.clone(),
            course_name: c.course.name.clone(),
            class_name: Some(c.clazz.name.clone()),
            teacher_name: c
                .creater
                .as_ref()
                .map(|cr| cr.full_name.clone())
                .unwrap_or_default(),
            course_status: c.status.clone().unwrap_or_default(),
            create_time: c.create_time.clone(),
            resource_state: results.get(&c.id).cloned().unwrap_or_default(),
        })
        .collect();

    Ok(DashboardState {
        courses: course_summaries,
    })
}

pub async fn complete_course_resources(
    app: &AppHandle,
    ccid: &str,
) -> Result<CompletionResult, String> {
    let stored =
        load_stored_session(app)?.ok_or_else(|| "未找到登录会话，请重新登录".to_string())?;

    if !stored.has_token() {
        return Err("当前没有可用的登录令牌，请重新登录".to_string());
    }

    let mut client = MosoteachClient::new();
    client.restore_session(&stored);

    let resources = client.list_resources(ccid).await?;

    // ==================== 调试信息开始 ====================
    println!("\n========== [Complete] 调试信息 ==========");
    println!("[Complete] 课程ID (ccid): {}", ccid);
    println!("[Complete] 获取到资源数量: {}", resources.len());

    // 打印每个资源的完整结构（用于调试字段缺失问题）
    println!("\n[Complete] 资源列表详细结构:");
    for (i, r) in resources.iter().enumerate() {
        println!("  资源[{}]: id={}", i, r._id);
        println!(
            "    - score: {:?}, obtain_score: {:?}",
            r.score, r.obtain_score
        );
        println!(
            "    - mime_type: {:?}, meta_duration: {:?}",
            r.mime_type, r.meta_duration
        );
        println!(
            "    - full_cover_url: {:?}",
            r.full_cover_url.as_ref().map(|u| if u.len() > 80 {
                format!("{}...({} chars)", &u[..80], u.len())
            } else {
                u.clone()
            })
        );
    }

    // 检查 full_cover_url 是否存在
    let resources_with_url: Vec<_> = resources
        .iter()
        .filter(|r| r.full_cover_url.is_some())
        .collect();
    let resources_without_url: Vec<_> = resources
        .iter()
        .filter(|r| r.full_cover_url.is_none())
        .collect();

    println!(
        "\n[Complete] URL 统计: 有URL的={}, 无URL的={}",
        resources_with_url.len(),
        resources_without_url.len()
    );

    if !resources_without_url.is_empty() {
        println!("[Complete] 无URL的资源ID列表:");
        for r in &resources_without_url {
            println!("    - {}", r._id);
        }
    }
    // ==================== 调试信息结束 ====================

    println!("\n[Complete] 开始处理资源...");

    let incomplete: Vec<&ResourceItem> = resources
        .iter()
        .filter(|r| {
            let obtain = r.obtain_score.unwrap_or(-1.0);
            let score = r.score.unwrap_or(0.0);
            obtain < 0.0 || obtain < score
        })
        .collect();

    let total = incomplete.len();
    println!("[Complete] 未完成资源数: {}", total);

    if total == 0 {
        println!("[Complete] 没有未完成资源，跳过");
        return Ok(CompletionResult {
            total: 0,
            completed: 0,
            failed: vec![],
        });
    }

    println!("\n[Complete] 开始标记资源为已完成...");

    // 使用正确的 viewer API 来标记资源为已完成
    let sem = Arc::new(Semaphore::new(5));
    let mut handles = Vec::new();

    for resource in &incomplete {
        let ccid = ccid.to_string();
        let resource_id = resource._id.clone();
        let mime_type = resource.mime_type.clone().unwrap_or_default();
        let meta_duration = resource.meta_duration.unwrap_or(0);
        let _full_cover_url = resource.full_cover_url.clone();
        let client = client.clone();
        let sem = sem.clone();

        handles.push(async move {
            let _permit = sem.acquire().await.unwrap();

            // 1. 视频资源需要模拟真实播放
            if mime_type.starts_with("video/") && meta_duration > 0 {
                // 获取 viewer URL (包含 m3u8 播放列表地址)
                match client.get_viewer_url(&ccid, &resource_id).await {
                    Ok(viewer) => {
                        if let Some(url) = viewer.url {
                            // 清理 URL (移除 oss 签名参数)
                            let clean_url = clean_cdn_url(&url);
                            println!("[Complete] 获取到视频URL: {}", clean_url);

                            // 获取 m3u8 播放列表
                            match client.fetch_m3u8(&clean_url).await {
                                Ok(m3u8_content) => {
                                    // 简单解析: 查找所有 .ts 分片
                                    let ts_count = m3u8_content.matches(".ts").count();
                                    if ts_count > 0 {
                                        println!(
                                            "[Complete] 发现 {} 个视频分片，请求中...",
                                            ts_count
                                        );

                                        // 获取 base URL (用于拼接分片地址)
                                        let base = if let Some(idx) = clean_url.rfind('/') {
                                            format!("{}/", &clean_url[..idx + 1])
                                        } else {
                                            clean_url.trim_end_matches(".m3u8").to_string()
                                        };

                                        // 请求所有分片
                                        let _ = client.fetch_ts_segments(&base, ts_count).await;
                                        println!("[Complete] 分片请求完成");
                                    }
                                }
                                Err(e) => {
                                    println!("[Complete] 获取m3u8失败: error={}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("[Complete] 获取viewer URL失败: error={}", e);
                    }
                }

                // 更新观看进度为视频长度，发送多次确保完成
                let duration = meta_duration as f64;
                let target_watch = meta_duration;
                if let Err(e) = client
                    .report_progress_multiple(&ccid, &resource_id, target_watch, duration, 3)
                    .await
                {
                    println!(
                        "[Complete] 进度上报失败: resource_id={}, error={}",
                        resource_id, e
                    );
                }
            }

            // 2. 非视频资源，优先使用 view_resource (预览 API)
            if !mime_type.starts_with("video/") {
                println!("[Complete] 开始处理非视频资源: resource_id={}, mime_type={}", resource_id, mime_type);

                // 优先使用 view_resource (预览 API)
                match client.view_resource(&ccid, &resource_id).await {
                    Ok(_) => {
                        println!("[Complete] ✓ 预览完成(view_resource): resource_id={}", resource_id);
                    }
                    Err(e) => {
                        println!("[Complete] ✗ view_resource 失败，准备尝试 download: resource_id={}, error={}", resource_id, e);

                        // download API 作为备用
                        match client.download_resource_api(&ccid, &resource_id).await {
                            Ok(_) => {
                                println!("[Complete] ✓ 下载完成(download API): resource_id={}", resource_id);
                            }
                            Err(e2) => {
                                println!("[Complete] ✗ download API 也失败: resource_id={}, error={}", resource_id, e2);
                            }
                        }
                    }
                }
            }
        });
    }

    println!("[Complete] 等待所有处理完成...");
    join_all(handles).await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    println!("[Complete] 重新获取资源列表...");
    let resources_after = match client.list_resources(ccid).await {
        Ok(r) => r,
        Err(e) => {
            println!("[Complete] 重新获取失败: {}", e);
            return Ok(CompletionResult {
                total,
                completed: 0,
                failed: incomplete.iter().map(|r| r._id.clone()).collect(),
            });
        }
    };

    let mut completed = 0usize;
    let mut failed = Vec::new();

    for before in &incomplete {
        let after = resources_after.iter().find(|r| r._id == before._id);

        match after {
            Some(after) => {
                let score = after.score.unwrap_or(0.0);
                let obtain = after.obtain_score.unwrap_or(-1.0);
                println!(
                    "[Complete] 对比 {}: score={} obtain={} {}",
                    before._id,
                    score,
                    obtain,
                    if score > 0.0 && obtain >= score {
                        "✓ 已完成"
                    } else {
                        "✗ 未完成"
                    }
                );
                if score > 0.0 && obtain >= score {
                    completed += 1;
                } else {
                    failed.push(before._id.clone());
                }
            }
            None => {
                println!("[Complete] 对比 {}: 刷新后未找到该资源", before._id);
                failed.push(before._id.clone());
            }
        }
    }

    println!("[Complete] 完成统计: {}/{} 成功", completed, total);
    Ok(CompletionResult {
        total,
        completed,
        failed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_session_round_trip_preserves_token_and_username() {
        let session = StoredSession {
            username: "student".to_string(),
            user: Some(SessionUser {
                user_id: "u1".to_string(),
                full_name: "Test User".to_string(),
                school_name: Some("School".to_string()),
                student_no: Some("1001".to_string()),
                department_name: Some("Design".to_string()),
            }),
            user_id: Some("u1".to_string()),
            token: Some("test_token".to_string()),
        };

        let json = serde_json::to_string(&session).unwrap();
        let restored: StoredSession = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.username, "student");
        assert!(restored.has_token());
        assert_eq!(restored.token.as_deref(), Some("test_token"));
    }

    #[test]
    fn course_list_response_deserializes() {
        let payload = r#"{
          "clazzCourses": [
            {
              "id": "course-1",
              "course": {
                "name": "现代设计史",
                "id": "c-1"
              },
              "clazz": {
                "name": "环设23级",
                "id": "clazz-1"
              },
              "fullCoverUrl": "https://example.com/cover.png",
              "creater": {
                "fullName": "李媛",
                "userId": "teacher-1"
              },
              "roleId": 2
            }
          ]
        }"#;

        let response: CourseListApiResponse = serde_json::from_str(payload).unwrap();

        assert_eq!(response.clazz_courses.len(), 1);
        assert_eq!(response.clazz_courses[0].course.name, "现代设计史");
        assert_eq!(response.clazz_courses[0].clazz.name, "环设23级");
    }

    #[test]
    fn resource_state_defaults_to_zero() {
        let state = ResourceState::default();

        assert_eq!(state.completed, 0);
        assert_eq!(state.incomplete, 0);
    }
}

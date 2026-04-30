use crate::api::{URL_CC_LIST_JOINED, URL_LOGIN};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

const SESSION_FILE_NAME: &str = "session.json";

pub struct MosoteachClient {
    http_client: Client,
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
    headers.insert(
        "x-client-app-id",
        HeaderValue::from_static("MTWEB"),
    );
    headers.insert(
        "x-client-version",
        HeaderValue::from_static("6.0.0"),
    );
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
    pub clazzCourseId: String,
    #[serde(alias = "courseName", alias = "course_name")]
    pub courseName: String,
    #[serde(alias = "className", alias = "class_name")]
    pub className: Option<String>,
    #[serde(alias = "teacherName", alias = "teacher_name")]
    pub teacherName: String,
    #[serde(alias = "courseStatus", alias = "course_status")]
    pub courseStatus: String,
    #[serde(alias = "createTime", alias = "create_time")]
    pub createTime: Option<String>,
    pub resourceState: ResourceState,
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
        let school_name = user.bind_school.as_ref().and_then(|s| s.school_name.clone());
        let department_name = user.bind_school.as_ref().and_then(|s| s.department_name.clone());
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
            http_client,
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

    pub async fn login(&mut self, username: &str, ciphertext: &str) -> Result<(User, String), String> {
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

        println!("[Login] POST {}", url);
        println!("[Login] Body: account={}, ciphertext={}", username, ciphertext);

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|error| format!("网络请求失败: {}", error))?;

        let status = response.status();
        let body_text = response.text().await.map_err(|e| format!("读取响应失败: {}", e))?;

        println!("[Login] Response status: {}", status);
        println!("[Login] Response body: {}", body_text);

        if !body_text.starts_with('{') {
            return Err(format!("服务器返回非JSON响应 (状态码: {}): {}", status, body_text));
        }

        let login_response: LoginApiResponse = serde_json::from_str(&body_text)
            .map_err(|e| {
                println!("[Login] JSON parse error: {}", e);
                format!("JSON解析失败: {}", e)
            })?;

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

        println!("[ListCourse] GET {}", url);
        println!("[ListCourse] Using token: {}", token);

        let response = self
            .http_client
            .get(&url)
            .header("x-token", token)
            .send()
            .await
            .map_err(|error| {
                println!("[ListCourse] Request error: {}", error);
                error.to_string()
            })?;

        let status = response.status();

        let body_text = response.text().await.map_err(|e| {
            println!("[ListCourse] Read body error: {}", e);
            e.to_string()
        })?;

        println!("[ListCourse] Response status: {}", status);
        println!("[ListCourse] Response body: {}", body_text);

        let course_response: CourseListApiResponse = serde_json::from_str(&body_text)
            .map_err(|error| {
                println!("[ListCourse] JSON parse error: {}", error);
                error.to_string()
            })?;

        println!("[ListCourse] Parsed {} courses", course_response.clazz_courses.len());

        Ok(course_response.clazz_courses)
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
            save_stored_session(app, &StoredSession::username_only(remembered_username.clone()))?;
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
    let stored_session = load_stored_session(app)?
        .ok_or_else(|| "未找到登录会话，请重新登录".to_string())?;

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
    let session = serde_json::from_str::<StoredSession>(&content).map_err(|error| error.to_string())?;
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

async fn build_dashboard(client: &MosoteachClient) -> Result<DashboardState, String> {
    let courses = client.list_course().await?;
    let course_summaries = courses
        .into_iter()
        .map(|c| CourseSummary {
            clazzCourseId: c.id,
            courseName: c.course.name,
            className: Some(c.clazz.name),
            teacherName: c.creater.as_ref().map(|cr| cr.full_name.clone()).unwrap_or_default(),
            courseStatus: c.status.unwrap_or_default(),
            createTime: c.create_time,
            resourceState: ResourceState::default(),
        })
        .collect();

    Ok(DashboardState {
        courses: course_summaries,
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
              "courseName": "现代设计史",
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
        assert_eq!(response.clazz_courses[0].course_name, "现代设计史");
    }

    #[test]
    fn resource_state_defaults_to_zero() {
        let state = ResourceState::default();

        assert_eq!(state.completed, 0);
        assert_eq!(state.incomplete, 0);
    }
}

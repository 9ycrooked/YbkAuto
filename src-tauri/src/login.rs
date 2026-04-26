use crate::api::{URL_CC_LIST_JOINED, URL_CHECKIN_OPEN, URL_LOGIN};
use crate::request_signer::{sign_request, RequestContext};
use indexmap::IndexMap;
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
const NO_OPEN_CHECKIN_MESSAGES: [&str; 3] = ["暂无签到", "未开启", "没有开启"];

pub fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Dalvik/2.1.0 (Linux; U; Android 8.1.0; ONE A2001 Build/OPM7.181205.001)",
        ),
    );
    headers.insert("X-scheme", HeaderValue::from_static("https"));
    headers.insert("X-app-id", HeaderValue::from_static("MTANDROID"));
    headers.insert("X-app-version", HeaderValue::from_static("5.1.1"));
    headers.insert("X-dpr", HeaderValue::from_static("2.7"));
    headers.insert("X-app-machine", HeaderValue::from_static("ONE A2001"));
    headers.insert("X-app-system-version", HeaderValue::from_static("8.1.0"));
    headers.insert("Host", HeaderValue::from_static("api.mosoteach.cn"));
    headers
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct LoginApiResponse {
    pub user: User,
    pub result_code: i64,
    pub result_msg: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct User {
    pub user_id: String,
    pub student_no: String,
    pub access_id: String,
    pub access_secret: String,
    pub full_name: String,
    pub student_score: String,
    pub student_level: String,
    pub phone_number: String,
    pub email_flag: String,
    pub last_sec_update_time: String,
    pub last_sec_update_ts_s: String,
    pub school_id: String,
    pub department_id: String,
    pub gender: String,
    pub profile_complete_flag: String,
    pub birth_date: String,
    pub gift_bean_count: i32,
    pub school_name: String,
    pub department_name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct CourseListApiResponse {
    #[serde(rename = "rows")]
    pub courses: Vec<CourseItem>,
    pub result_code: i64,
    pub result_msg: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CourseItem {
    pub id: String,
    pub status: String,
    pub invitation_code: String,
    pub cover_url: String,
    pub create_time: String,
    pub display_order: i64,
    pub course_create_time: String,
    pub from_mqp: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub term: Term,
    pub clazz: Clazz,
    pub course: Course,
    pub updated: Updated,
    pub creater: Creater,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Term {
    pub from: i64,
    pub to: i64,
    pub term: i64,
    pub title: String,
    pub is_current: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Clazz {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub create_time: String,
    pub display_order: i64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Updated {
    pub resource: String,
    pub notice: String,
    pub activity: String,
    pub member: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Creater {
    pub user_id: String,
    pub full_name: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct CurrentOpenApiResponse {
    pub result_code: i64,
    pub result_msg: String,
    #[serde(default)]
    pub data: Option<OpenCheckinInfo>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCheckinInfo {
    #[serde(rename = "checkinId", alias = "checkin_id")]
    pub checkin_id: String,
    pub title: String,
    #[serde(rename = "type", default)]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceState {
    pub status: String,
    pub label: String,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self {
            status: "unknown".to_string(),
            label: "待实现".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseSummary {
    pub clazz_course_id: String,
    pub course_name: String,
    pub class_name: String,
    pub teacher_name: String,
    pub course_status: String,
    pub checkin_state: String,
    pub open_checkin: Option<OpenCheckinInfo>,
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
    pub school_name: String,
    pub student_no: String,
    pub department_name: String,
}

impl From<&User> for SessionUser {
    fn from(user: &User) -> Self {
        Self {
            user_id: user.user_id.clone(),
            full_name: user.full_name.clone(),
            school_name: user.school_name.clone(),
            student_no: user.student_no.clone(),
            department_name: user.department_name.clone(),
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
    access_id: Option<String>,
    #[serde(default)]
    access_secret: Option<String>,
    #[serde(default)]
    last_sec_update_ts_s: Option<String>,
}

impl StoredSession {
    fn username_only(username: String) -> Self {
        Self {
            username,
            ..Default::default()
        }
    }

    fn has_token(&self) -> bool {
        self.user_id.is_some()
            && self.access_id.is_some()
            && self.access_secret.is_some()
            && self.last_sec_update_ts_s.is_some()
    }
}

pub struct MosoteachClient {
    http_client: Client,
    user: Option<User>,
    user_id: Option<String>,
    access_id: Option<String>,
    access_secret: Option<String>,
    last_sec_update_ts_s: Option<String>,
}

impl MosoteachClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .default_headers(base_headers())
            .build()
            .expect("无法创建 HTTP 客户端");

        Self {
            http_client,
            user: None,
            user_id: None,
            access_id: None,
            access_secret: None,
            last_sec_update_ts_s: None,
        }
    }

    fn restore_session(&mut self, session: &StoredSession) {
        self.user_id = session.user_id.clone();
        self.access_id = session.access_id.clone();
        self.access_secret = session.access_secret.clone();
        self.last_sec_update_ts_s = session.last_sec_update_ts_s.clone();
    }

    fn to_stored_session(&self, username: String) -> StoredSession {
        StoredSession {
            username,
            user: self.user.as_ref().map(SessionUser::from),
            user_id: self.user_id.clone(),
            access_id: self.access_id.clone(),
            access_secret: self.access_secret.clone(),
            last_sec_update_ts_s: self.last_sec_update_ts_s.clone(),
        }
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<User, String> {
        let mut form = IndexMap::new();
        form.insert("account_name", username);
        form.insert("app_id", "MTANDROID");
        form.insert("app_version_name", "5.1.1");
        form.insert("app_version_number", "111");
        form.insert("device_type", "ANDROID");
        form.insert("dpr", "2.7");
        form.insert("system_version", "8.1.0");
        form.insert("user_pwd", password);

        let (time, sign) = sign_request(&RequestContext::new(URL_LOGIN).with_form(&form, true));

        let response = self
            .http_client
            .post(URL_LOGIN)
            .header("Date", &time)
            .header("X-mssvc-signature", &sign)
            .form(&form)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        let login_response: LoginApiResponse = response
            .json()
            .await
            .map_err(|error| error.to_string())?;

        if login_response.result_code != 0 {
            return Err(login_response.result_msg);
        }

        self.user = Some(login_response.user.clone());
        self.user_id = Some(login_response.user.user_id.clone());
        self.access_id = Some(login_response.user.access_id.clone());
        self.access_secret = Some(login_response.user.access_secret.clone());
        self.last_sec_update_ts_s = Some(login_response.user.last_sec_update_ts_s.clone());

        Ok(login_response.user)
    }

    pub async fn list_course(&self) -> Result<Vec<CourseItem>, String> {
        let user_id = self
            .user_id
            .as_deref()
            .ok_or_else(|| "user_id 未登录".to_string())?;
        let access_secret = self
            .access_secret
            .as_deref()
            .ok_or_else(|| "access_secret 未登录".to_string())?;
        let access_id = self
            .access_id
            .as_deref()
            .ok_or_else(|| "access_id 未登录".to_string())?;
        let sec_ts = self
            .last_sec_update_ts_s
            .as_deref()
            .ok_or_else(|| "last_sec_update_ts_s 未登录".to_string())?;

        let (time, sign) = sign_request(
            &RequestContext::new(URL_CC_LIST_JOINED)
                .with_user_id(user_id)
                .with_secret(access_secret),
        );

        let response = self
            .http_client
            .post(URL_CC_LIST_JOINED)
            .header("Date", &time)
            .header("X-mssvc-access-id", access_id)
            .header("X-mssvc-signature", &sign)
            .header("X-mssvc-sec-ts", sec_ts)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        let course_response: CourseListApiResponse = response
            .json()
            .await
            .map_err(|error| error.to_string())?;

        if course_response.result_code != 0 {
            return Err(course_response.result_msg);
        }

        Ok(course_response.courses)
    }

    pub async fn current_open_checkin(
        &self,
        clazz_course_id: &str,
    ) -> Result<Option<OpenCheckinInfo>, String> {
        let user_id = self
            .user_id
            .as_deref()
            .ok_or_else(|| "user_id 未登录".to_string())?;
        let access_secret = self
            .access_secret
            .as_deref()
            .ok_or_else(|| "access_secret 未登录".to_string())?;
        let access_id = self
            .access_id
            .as_deref()
            .ok_or_else(|| "access_id 未登录".to_string())?;
        let sec_ts = self
            .last_sec_update_ts_s
            .as_deref()
            .ok_or_else(|| "last_sec_update_ts_s 未登录".to_string())?;

        let mut form = IndexMap::new();
        form.insert("clazz_course_id", clazz_course_id);

        let (time, sign) = sign_request(
            &RequestContext::new(URL_CHECKIN_OPEN)
                .with_form(&form, false)
                .with_user_id(user_id)
                .with_secret(access_secret),
        );

        let response = self
            .http_client
            .post(URL_CHECKIN_OPEN)
            .header("Date", &time)
            .header("X-mssvc-access-id", access_id)
            .header("X-mssvc-signature", &sign)
            .header("X-mssvc-sec-ts", sec_ts)
            .form(&form)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        let current_open_response: CurrentOpenApiResponse = response
            .json()
            .await
            .map_err(|error| error.to_string())?;

        if current_open_response.result_code == 0 {
            return Ok(current_open_response.data);
        }

        if NO_OPEN_CHECKIN_MESSAGES
            .iter()
            .any(|message| current_open_response.result_msg.contains(message))
        {
            return Ok(None);
        }

        Err(current_open_response.result_msg)
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
    password: String,
) -> Result<SessionState, String> {
    let username = username.trim().to_string();

    if username.is_empty() {
        return Err("请输入账号".to_string());
    }

    if password.is_empty() {
        return Err("请输入密码".to_string());
    }

    save_stored_session(app, &StoredSession::username_only(username.clone()))?;

    let mut client = MosoteachClient::new();
    let user = client.login(&username, &password).await?;
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
    let mut course_summaries = Vec::with_capacity(courses.len());

    for course in courses {
        let (checkin_state, open_checkin) = match client.current_open_checkin(&course.id).await {
            Ok(Some(open_checkin)) => ("open".to_string(), Some(open_checkin)),
            Ok(None) => ("closed".to_string(), None),
            Err(_) => ("error".to_string(), None),
        };

        course_summaries.push(CourseSummary {
            clazz_course_id: course.id,
            course_name: course.course.name,
            class_name: course.clazz.name,
            teacher_name: course.creater.full_name,
            course_status: course.status,
            checkin_state,
            open_checkin,
            resource_state: ResourceState::default(),
        });
    }

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
                school_name: "School".to_string(),
                student_no: "1001".to_string(),
                department_name: "Design".to_string(),
            }),
            user_id: Some("u1".to_string()),
            access_id: Some("aid".to_string()),
            access_secret: Some("secret".to_string()),
            last_sec_update_ts_s: Some("123456".to_string()),
        };

        let json = serde_json::to_string(&session).unwrap();
        let restored: StoredSession = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.username, "student");
        assert!(restored.has_token());
        assert_eq!(restored.access_secret.as_deref(), Some("secret"));
    }

    #[test]
    fn course_list_response_deserializes() {
        let payload = r#"{
          "result_code": 0,
          "result_msg": "OK",
          "rows": [
            {
              "id": "course-1",
              "status": "OPEN",
              "invitation_code": "123456",
              "cover_url": "https://example.com/cover.png",
              "create_time": "2026-03-24 18:49:56",
              "display_order": -1,
              "course_create_time": "2025-04-14 09:32:51",
              "from_mqp": "N",
              "start_time": null,
              "end_time": null,
              "term": {
                "from": 2025,
                "to": 2026,
                "term": 2,
                "title": "2025-2026-2",
                "is_current": "Y"
              },
              "clazz": {
                "id": "clazz-1",
                "name": "环设23级"
              },
              "course": {
                "id": "c-1",
                "name": "现代设计史",
                "create_time": "2025-04-14 09:32:51",
                "display_order": -1
              },
              "updated": {
                "resource": "N",
                "notice": "N",
                "activity": "N",
                "member": "N"
              },
              "creater": {
                "user_id": "teacher-1",
                "full_name": "李媛",
                "avatar_url": "https://example.com/avatar.jpg"
              }
            }
          ]
        }"#;

        let response: CourseListApiResponse = serde_json::from_str(payload).unwrap();

        assert_eq!(response.courses.len(), 1);
        assert_eq!(response.courses[0].course.name, "现代设计史");
    }

    #[test]
    fn current_open_response_deserializes() {
        let payload = r#"{
          "result_code": 0,
          "result_msg": "OK",
          "data": {
            "checkin_id": "checkin-1",
            "title": "课堂签到",
            "type": "NORMAL"
          }
        }"#;

        let response: CurrentOpenApiResponse = serde_json::from_str(payload).unwrap();

        assert_eq!(response.data.unwrap().checkin_id, "checkin-1");
    }

    #[test]
    fn resource_state_defaults_to_placeholder() {
        let state = ResourceState::default();

        assert_eq!(state.status, "unknown");
        assert_eq!(state.label, "待实现");
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(super) struct SessionResponse {
    #[serde(rename = "SessionId")]
    pub(super) session_id: u64,
    #[serde(rename = "SessionName")]
    pub(super) name: String,
    #[serde(rename = "SessionDescription")]
    pub(super) description: String,
    #[serde(rename = "StartDateTime")]
    pub(super) start: String,
    #[serde(rename = "EndDateTime")]
    pub(super) end: String,
    #[serde(rename = "SessionStatus")]
    pub(super) session_status: String,
    #[serde(rename = "Locations")]
    pub(super) locations: Option<Vec<String>>,
    #[serde(rename = "AttendanceStatus")]
    pub(super) attendance_status: String,
}

#[derive(Serialize)]
pub struct Session {
    pub session_id: u64,
    pub name: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub locations: Vec<String>,
    pub session_status: String,
    pub attendance_status: String,
}

impl From<SessionResponse> for Session {
    fn from(session: SessionResponse) -> Self {
        Session {
            session_id: session.session_id,
            name: session.name,
            description: session.description,
            start: session.start,
            end: session.end,
            locations: session.locations.unwrap_or_default(),
            session_status: session.session_status,
            attendance_status: session.attendance_status,
        }
    }
}

#[derive(Serialize)]
pub(super) struct RegisterSessionRequest {
    #[serde(rename = "SessionId")]
    pub(super) session_id: u64,
    #[serde(rename = "SessionRegistrationCode")]
    pub(super) registration_code: String,
    #[serde(rename = "ForceIncorrectSessionRegistration")]
    pub(super) force_incorrect_session_registration: bool,
}

#[derive(Deserialize)]
pub(super) struct RegisterSessionResponse {
    #[serde(rename = "Status")]
    pub(super) status: String,
    #[serde(rename = "OK")]
    pub(super) ok: bool,
}

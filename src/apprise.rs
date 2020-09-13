use serde::Serialize;
use std::env;
use url::{ParseError, Url};

use crate::grafana::{GrafanaPayload, GrafanaState};

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AppriseState {
    Info,
    Success,
    Warning,
    Failure,
}

impl From<GrafanaState> for AppriseState {
    fn from(gf_state: GrafanaState) -> AppriseState {
        return match gf_state {
            GrafanaState::Ok => AppriseState::Success,
            GrafanaState::Paused => AppriseState::Info,
            GrafanaState::Alerting => AppriseState::Failure,
            GrafanaState::Pending => AppriseState::Info,
            GrafanaState::NoData => AppriseState::Warning,
        };
    }
}

#[derive(Serialize, Debug)]
pub struct ApprisePayload {
    pub title: String,
    pub body: String,

    #[serde(rename = "type")]
    pub notification_type: AppriseState,
}

impl From<GrafanaPayload> for ApprisePayload {
    fn from(gf_payload: GrafanaPayload) -> ApprisePayload {
        return ApprisePayload {
            title: gf_payload.title,
            body: gf_payload.message,
            notification_type: AppriseState::from(gf_payload.state),
        };
    }
}

pub fn get_apprise_url(key: &str) -> Result<Url, ParseError> {
    let apprise_host =
        env::var("APPRISE_HOST").unwrap_or_else(|_| String::from("http://apprise:8000"));
    let url = Url::parse(&apprise_host)?;
    return url.join(&format!("/notify/{}", key));
}
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GrafanaState {
    Ok,
    Paused,
    Alerting,
    Pending,
    NoData,
}

#[derive(Deserialize, Debug)]
pub struct GrafanaPayload {
    title: String,
    message: String,
    state: GrafanaState,

    #[serde(rename = "imageUrl")]
    image: String,

    tags: HashMap<String, String>,
}

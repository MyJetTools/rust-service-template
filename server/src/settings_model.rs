use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "RustServiceTemplateTest")]
    pub inner: SettingsModelInner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModelInner {
    #[serde(rename = "ZipkinUrl")]
    pub zipkin_url: String,

    #[serde(rename = "SeqServiceUrl")]
    pub seq_service_url: String,

    #[serde(rename = "LogStashUrl")]
    pub log_stash_url: String,
}

impl rust_service_sdk::app::app_ctx::GetLogStashUrl for SettingsModel {
    fn get_logstash_url(&self) -> String {
        self.inner.log_stash_url.clone()
        //USE ONLY CONSOLE SINK"".to_string()
    }
}
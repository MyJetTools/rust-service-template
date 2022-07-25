use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    #[serde(rename = "RustServiceTemplateTest")]
    pub inner: SettingsModelInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModelInner {
    #[serde(rename = "ZipkinUrl")]
    pub zipkin_url: String,

    #[serde(rename = "SeqServiceUrl")]
    pub seq_service_url: String,

    #[serde(rename = "LogStashUrl")]
    pub log_stash_url: String,
}

impl crate::app::app_ctx::GetLogStashUrl for SettingsModel {
    fn get_logstash_url(&self) -> String {
        self.inner.log_stash_url.clone()
    }
}
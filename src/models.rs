use crate::lh_models::{AuditValue, Throttling};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PageScoreParameters {
    pub url: String,
    pub throttling: Option<String>,
    pub attempts: Option<i8>,
    #[serde(rename = "lighthouseVersion")]
    pub lighthouse_version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PageScoreReport {
    #[serde(rename = "lighthouseVersion")]
    pub lighthouse_version: String,

    #[serde(rename = "requestedUrl")]
    requested_url: String,

    #[serde(rename = "finalUrl")]
    final_url: String,

    #[serde(rename = "fetchTime")]
    fetch_time: String,

    #[serde(rename = "categories")]
    categories: Categories,
    #[serde(rename = "keyAuditMeasures")]
    key_audits: KeyAudits,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct KeyAudits {
    #[serde(rename = "firstContentfulPaint")]
    first_contentful_paint: AuditValue,

    #[serde(rename = "speedIndex")]
    speed_index: AuditValue,

    #[serde(rename = "largestContentfulPaint")]
    largest_contentful_paint: Option<AuditValue>,

    #[serde(rename = "interactive")]
    interactive: AuditValue,

    #[serde(rename = "totalBlockingTime")]
    total_blocking_time: AuditValue,

    #[serde(rename = "cumulativeLayoutShift")]
    cumulative_layout_shift: Option<AuditValue>,
    //LH5 metrics; but not in LH6
    #[serde(rename = "firstMeaningfulPaint")]
    first_meaningful_paint: AuditValue,

    #[serde(rename = "firstCpuIdle")]
    first_cpu_idle: AuditValue,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Categories {
    #[serde(rename = "performance")]
    performance: Performance,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Performance {
    id: String,
    title: String,
    score: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ConfigSettings {
    #[serde(rename = "throttlingMethod")]
    throttling_method: String,

    #[serde(rename = "throttling")]
    throttling: Throttling,

    #[serde(rename = "emulatedFormFactor")]
    emulated_form_factor: String,
}

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AllAttemptReports {
	#[serde(rename = "bestScore")]
	best_score: f64,

	#[serde(rename = "bestScoreIndex")]
	best_score_index: usize,

	#[serde(rename = "results")]
	reports: Vec<Report>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Report {
	#[serde(rename = "lighthouseVersion")]
	lighthouse_version: String,

	#[serde(rename = "requestedUrl")]
	requested_url: String,

	#[serde(rename = "finalUrl")]
	final_url: String,

	#[serde(rename = "fetchTime")]
	fetch_time: String,

	#[serde(rename = "environment")]
	environment: Environment,

	audits: Audits,

	#[serde(rename = "configSettings")]
	config_settings: ConfigSettings,

	#[serde(rename = "categories")]
	categories: Categories,
	//TODOs
	//categoryGroups
	//timing
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Environment {}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Audits {
	#[serde(rename = "first-contentful-paint")]
	first_contentful_paint: AuditValue,

	#[serde(rename = "largest-contentful-paint")]
	largest_contentful_paint: Option<AuditValue>,

	#[serde(rename = "first-meaningful-paint")]
	first_meaningful_paint: AuditValue,
	#[serde(rename = "speed-index")]
	speed_index: AuditValue,

	#[serde(rename = "total-blocking-time")]
	total_blocking_time: AuditValue,

	#[serde(rename = "max-potential-fid")]
	max_potential_fid: AuditValue,

	#[serde(rename = "cumulative-layout-shift")]
	cumulative_layout_shift: Option<AuditValue>,

	#[serde(rename = "server-response-time")]
	server_response_time: Option<AuditValue>, // TODO: More fields
	#[serde(rename = "first-cpu-idle")]
	first_cpu_idle: AuditValue,

	#[serde(rename = "interactive")]
	interactive: AuditValue,

	#[serde(rename = "network-requests")]
	network_requests: NetworkRequests,

	#[serde(rename = "network-rtt")]
	network_rtt: NetworkRtt,

	#[serde(rename = "network-server-latency")]
	network_server_latency: NetworkServerLatency,

	#[serde(rename = "main-thread-tasks")]
	main_thread_tasks: MainThreadTasks,

	#[serde(rename = "metrics")]
	metrics: Metrics,

	#[serde(rename = "resource-summary")]
	resource_summary: ResourceSummary,

	#[serde(rename = "third-party-summary")]
	third_party_summary: ThirdPartySummary,
	//TODOs
	//largest-contentful-paint-element
	//layout-shift-elements
	//uses-long-cache-ttl
	//total-byte-weight
	//offscreen-images
	//render-blocking-resources
	//unminified-css
	//unminified-javascript
	//unused-css-rules"
	//unused-javascript
	//uses-webp-images
	//uses-optimized-images
	//uses-text-compression
	//uses-responsive-images
	//efficient-animated-content
	//dom-size
	//no-document-write
	//uses-http2
	//uses-passive-event-listeners
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditValue {
	id: String,
	title: String,
	description: String,
	score: f64,

	#[serde(rename = "scoreDisplayMode")]
	score_display_mode: String,

	#[serde(rename = "numericValue")]
	numeric_value: Option<f64>,

	#[serde(rename = "numericUnit")]
	numeric_unit: Option<String>,

	#[serde(rename = "displayValue")]
	display_value: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRequests {
	id: String,
	title: String,
	description: String,
	score: Option<f64>,
	details: NetworkRequestDetails,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRequestDetails {
	headings: Vec<Heading>,
	items: Vec<NetworkRequestDetailsItem>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Heading {
	key: String,

	#[serde(rename = "itemType")]
	item_type: String,

	#[serde(rename = "displayUnit")]
	display_unit: Option<String>,

	text: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRequestDetailsItem {
	url: String,

	#[serde(rename = "startTime")]
	start_time: f64,

	#[serde(rename = "endTime")]
	end_time: f64,

	#[serde(rename = "finished")]
	finished: Option<bool>,

	#[serde(rename = "transferSize")]
	transfer_size: Option<i64>,

	#[serde(rename = "resourceSize")]
	resource_size: i64,

	#[serde(rename = "statusCode")]
	status_code: i16,

	#[serde(rename = "mimeType")]
	mime_type: String,

	#[serde(rename = "resourceType")]
	resource_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRtt {
	id: String,
	title: String,
	description: String,
	score: Option<f64>,
	details: NetworkRttDetails,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRttDetails {
	headings: Vec<Heading>,
	items: Vec<NetworkRttDetailsItem>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkRttDetailsItem {
	origin: String,
	rtt: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct NetworkServerLatency {
	//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct MainThreadTasks {
	//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Metrics {
	id: String,
	title: String,
	description: String,
	score: Option<f64>,
	#[serde(rename = "scoreDisplayMode")]
	score_display_mode: String,
	#[serde(rename = "numericValue")]
	numeric_value: f64,

	#[serde(rename = "numericUnit")]
	numeric_unit: Option<String>,

	#[serde(rename = "details")]
	details: MetricsDetails,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct MetricsDetails {
	//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ResourceSummary {
	id: String,
	title: String,
	description: String,
	score: Option<f64>,
	#[serde(rename = "displayValue")]
	display_value: String,
	#[serde(rename = "details")]
	details: ResourceSummaryDetails,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ResourceSummaryDetails {
	headings: Vec<Heading>,
	items: Vec<ResourceSummaryDetailsItem>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ResourceSummaryDetailsItem {
	#[serde(rename = "resourceType")]
	resource_type: String,
	#[serde(rename = "label")]
	label: String,
	#[serde(rename = "requestCount")]
	request_count: i32,
	#[serde(rename = "transferSize")]
	transfer_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ThirdPartySummary {
	//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ConfigSettings {
	#[serde(rename = "throttlingMethod")]
	throttling_method: String,

	#[serde(rename = "throttling")]
	throttling: Throttling,

	#[serde(rename = "auditMode")]
	audit_mode: bool,

	#[serde(rename = "gatherMode")]
	gather_mode: bool,

	#[serde(rename = "disableStorageReset")]
	disable_storage_reset: bool,

	#[serde(rename = "emulatedFormFactor")]
	emulated_form_factor: String,

	#[serde(rename = "internalDisableDeviceScreenEmulation")]
	internal_disable_device_screen_emulation: Option<bool>,

	channel: String,
	//budgets: String, //TODO
	locale: String,
	//blockedUrlPatterns: String, //TODO
	//additionalTraceCategories: String,
	//extraHeaders
	//precomputedLanternData
	//onlyAudits
	#[serde(rename = "onlyCategories")]
	only_categories: Vec<String>,
	//#[serde(rename = "skipAudits")]
	//skip_audits: Option<bool>,//TODO
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Throttling {
	#[serde(rename = "rttMs")]
	rtt_ms: i32,

	#[serde(rename = "throughputKbps")]
	throughput_kbps: f64,

	#[serde(rename = "requestLatencyMs")]
	request_latency_ms: f64,

	#[serde(rename = "downloadThroughputKbps")]
	download_throughput_kbps: f64,

	#[serde(rename = "uploadThroughputKbps")]
	upload_throughput_kbps: i16,

	#[serde(rename = "cpuSlowdownMultiplier")]
	cpu_slowdown_multiplier: i16,
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

	#[serde(rename = "auditRefs")]
	audit_refs: Vec<AuditRef>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditRef {
	id: String,
	weight: i8,
	group: Option<String>,
}

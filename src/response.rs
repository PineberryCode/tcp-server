#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct Response {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}
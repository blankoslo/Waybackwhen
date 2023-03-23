#[derive(Debug)]
pub struct SlackConfig {
    pub token: String,
    pub channel_id: String,
    pub dry_run: bool,
}

use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct SlackConfig {
    pub token: String,
    pub channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackRequest {
    pub channel: String,
    pub text: String,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadImageReq {
    pub channels: Vec<String>,
    pub file: String,
    pub filename: String,
    pub title: String,
    pub filetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub r#type: String,
    pub title: Title,
    pub image_url: String,
    pub alt_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    pub r#type: String,
    pub text: String,
    pub emoji: bool,
}

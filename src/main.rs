use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption::Png, Browser};
use reqwest::blocking::{multipart::Form, *};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug)]
struct SlackConfig {
    token: String,
    channel_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct SlackRequest {
    channel: String,
    text: String,
    blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UploadImageReq {
    channels: Vec<String>,
    file: String,
    filename: String,
    title: String,
    filetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    r#type: String,
    title: Title,
    image_url: String,
    alt_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Title {
    r#type: String,
    text: String,
    emoji: bool,
}

fn test_channel_conf() -> SlackConfig {
    let token = std::env::var("SLACK_TOKEN").unwrap();
    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    return SlackConfig { token, channel_id };
}

fn main() {
    let x = screenshot_site("https://blank.no");
    if x.is_err() {
        panic!("Error: {:?}", x);
    }
    upload_image(IMAGE_PATH);
    x.unwrap();
}

//const BASE_SLACK_URL: &str = "https://slack.com/api/";
const POST_MESSAGE: &str = "https://slack.com/api/chat.postMessage";
const IMAGE_PATH: &str = "/tmp/image.png";

/// Post a message to a slack channel
/// Currently unused
fn post_message(conf: &SlackConfig) -> () {
    let payload = SlackRequest {
        channel: conf.channel_id.to_string(),
        text: "Hei fra Rust".to_string(),
        blocks: block_payload("", ""),
    };
    let client = Client::new();
    let res = client
        .post(POST_MESSAGE)
        .bearer_auth(&conf.token)
        .json(&payload)
        .send();
    println!("{:?}", res);
}

/// Payload for posting a message to a slack channel
/// Currently unused
fn block_payload(title: &str, image_url: &str) -> Vec<Block> {
    let block = Block {
        r#type: "image".to_string(),
        title: Title {
            r#type: "plain_text".to_string(),
            text: title.to_string(),
            emoji: true,
        },
        image_url: image_url.to_string(),
        alt_text: title.to_string(),
    };
    return vec![block];
}

fn screenshot_site(url: &str) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;

    let png = tab.capture_screenshot(Png, Some(75), None, true)?;
    // write  the image to a file
    std::fs::write(IMAGE_PATH, png).unwrap();
    Ok(())
}

fn upload_image(image_path: &str) -> () {
    let conf = test_channel_conf();
    let form = reqwest::blocking::multipart::Form::new()
        .text("channels", conf.channel_id)
        .text("title", "Wikipedia beibi")
        .file("file", image_path)
        .unwrap();
    let client = Client::new();
    multipart_post(&client, form, "https://slack.com/api/files.upload");
}

fn slack_client() -> Client {
    let client = Client::new();
    client
}

fn multipart_post(client: &Client, form: Form, url: &str) -> () {
    let conf = test_channel_conf();
    let res = client
        .post(url)
        .bearer_auth(&conf.token)
        .multipart(form)
        .send()
        .unwrap();
    println!("{:?}", res.text());
}

mod structs;
use crate::structs::SlackRequest;
use chrono::{DateTime, TimeZone, Utc};
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption::Png, Browser};
use rand::rngs::ThreadRng;
use rand::Rng;
use reqwest::blocking::{multipart::Form, *};
use reqwest::header;
use std::error::Error;
use structs::{Block, SlackConfig, Title};

fn test_channel_conf() -> SlackConfig {
    let token = std::env::var("SLACK_TOKEN").unwrap();
    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    return SlackConfig { token, channel_id };
}
fn random_date(rng: &mut ThreadRng) -> DateTime<Utc> {
    //let start = 946684800; // 2000 
    let start = 1167631200; // 2007
    let end = 1420070400; // 2015
    let random_date = rng.gen_range(start..end);
    let date = Utc.timestamp_opt(random_date, 0).unwrap();
    return date;
}

fn date_to_wayback_url(date: DateTime<Utc>, url: &str) -> String {
    let date_str = date.format("%Y%m%d%H%M%S").to_string();
    let wayback_url = format!("https://web.archive.org/web/{}if_/{}", date_str, url);
    return wayback_url;
}

fn main() {
    let big_company_website_urls_that_existed_in_the00s = vec![
        "https://www.google.com",
        "https://www.facebook.com",
        "https://www.amazon.com",
        "https://www.apple.com",
        "https://www.microsoft.com",
        "https://www.netflix.com",
        "https://www.spotify.com",
        "https://www.twitter.com",
        "https://www.instagram.com",
        "https://www.youtube.com",
        "https://www.linkedin.com",
        "https://www.reddit.com",
        "https://www.tumblr.com",
        "https://www.pinterest.com",
        "https://www.whatsapp.com",
    ];
    // get random date between 2000 and 2015
    let mut rng = rand::thread_rng();
    let random_date = random_date(&mut rng);
    let random_url = big_company_website_urls_that_existed_in_the00s
        [rng.gen_range(0..big_company_website_urls_that_existed_in_the00s.len())];
    println!("Random date: {}", random_date);
    println!("Random url: {}", random_url);
    let url = date_to_wayback_url(random_date, random_url);

    let client = slack_client();
    let x = screenshot_site(&url);
    if x.is_err() {
        panic!("Error: {:?}", x);
    }
    upload_image(&client, IMAGE_PATH);
    x.unwrap();
}

//const BASE_SLACK_URL: &str = "https://slack.com/api/";
const IMAGE_PATH: &str = "/tmp/image.png";

fn screenshot_site(url: &str) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;

    let png = tab.capture_screenshot(Png, Some(75), None, true)?;
    // write  the image to a file
    std::fs::write(IMAGE_PATH, png).unwrap();
    Ok(())
}

fn upload_image(client: &Client, image_path: &str) -> () {
    let conf = test_channel_conf();
    let form = reqwest::blocking::multipart::Form::new()
        .text("channels", conf.channel_id)
        .text("title", "Wikipedia beibi")
        .file("file", image_path)
        .unwrap();
    multipart_post(&client, form, "https://slack.com/api/files.upload");
}

fn slack_client() -> Client {
    let conf = test_channel_conf();
    let mut headers = header::HeaderMap::new();

    let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", conf.token)).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    // get a client builder
    let client = Client::builder().default_headers(headers).build().unwrap();
    client
}

fn multipart_post(authed_client: &Client, form: Form, url: &str) -> () {
    let res = authed_client.post(url).multipart(form).send().unwrap();
    println!("{:?}", res.text());
}

/// UNUSED CODE
const POST_MESSAGE: &str = "https://slack.com/api/chat.postMessage";

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

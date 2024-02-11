use std::{str::FromStr, time::Duration};

use serde::Deserialize;

use analytical::*;
use tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};

pub enum HanabMessage {
    Welcome(WelcomeMessage),
}

#[derive(Debug, Deserialize)]
pub struct WelcomeMessage {
    username: String,
}

pub fn backoff<R, E: std::fmt::Display, F: Fn() -> Result<R, E>>(
    max_tries: u32,
    start_dur: Duration,
    f: F,
) -> Result<R, E> {
    let mut tries = 0;
    loop {
        let r = f();
        match r {
            Ok(v) => return Ok(v),
            Err(e) => {
                tries += 1;
                log::error!("Try {tries} FAILED: {e}");
                if tries >= max_tries {
                    return Err(e);
                }
                std::thread::sleep(start_dur * tries);
            }
        }
    }
}

impl FromStr for HanabMessage {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let msg_type = iter
            .next()
            .ok_or_else(|| color_eyre::eyre::eyre!("No msg type sent from server"))?;
        match msg_type {
            "welcome" => {
                let s = iter.fold(String::new(), |mut a, b| {
                    a.reserve(b.len() + 1);
                    a.push_str(b);
                    a.push_str(" ");
                    a
                });
                let data: WelcomeMessage = serde_json::from_str(s.trim_end())?;
                Ok(HanabMessage::Welcome(data))
            }
            v => Err(color_eyre::eyre::eyre!("unsupported message type: {v}")),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    pretty_env_logger::init();
    color_eyre::install()?;

    log::info!("Starting up...");

    let host = std::env::var("HANABI_HOST")?;
    let port: Option<u16> = match std::env::var("HANABI_PORT").ok() {
        Some(p) => Some(p.parse()?),
        None => None,
    };
    let username = std::env::var("HANABI_USERNAME")?;
    let password = std::env::var("HANABI_PASSWORD")?;
    let use_tls = if std::env::var("USE_TLS")? == "true" {
        "s"
    } else {
        ""
    };

    log::info!("Settings:");
    log::info!("\tHOST: {host}");
    if let Some(port) = port {
        log::info!("\tPORT: {port}");
    }
    log::info!("\tUSERNAME: {username}");
    log::info!("\tUSE_TLS: {use_tls}");

    let client = reqwest::blocking::Client::new();

    let login_body = serde_json::Value::Object({
        let mut m = serde_json::Map::new();
        m.insert("username".to_string(), serde_json::Value::String(username));
        m.insert("password".to_string(), serde_json::Value::String(password));
        m.insert(
            "version".to_string(),
            serde_json::Value::String("bot".to_string()),
        );

        m
    });
    log::debug!("Loging Req: {login_body}");

    let login = backoff(5, Duration::from_secs(5), || {
        client
            .post(format!(
                "http{}://{}{}/login",
                use_tls,
                host,
                match port {
                    Some(p) => format!(":{p}"),
                    None => "".to_string(),
                }
            ))
            .form(&login_body)
            .send()
    })?;

    let login_cookie = login
        .headers()
        .get("Set-Cookie")
        .ok_or_else(|| color_eyre::eyre::eyre!("Could not log in successfully"))?
        .as_bytes();

    let mut ws_request = format!(
        "ws{}://{}{}/ws",
        use_tls,
        host,
        match port {
            Some(p) => format!(":{p}"),
            None => "".to_string(),
        }
    )
    .into_client_request()?;
    ws_request
        .headers_mut()
        .insert("Cookie", HeaderValue::from_bytes(login_cookie)?);

    let (mut socket, _) = tungstenite::connect(ws_request)?;

    while let Ok(msg) = socket.read() {
        if let Message::Text(msg) = msg {
            log::debug!("Message received: {msg}");
            match HanabMessage::from_str(&msg) {
                Ok(msg) => match msg {
                    HanabMessage::Welcome(msg) => {
                        log::info!("Username: {}", msg.username)
                    }
                },
                Err(e) => {
                    log::error!("Error receiving msg: {e}");
                }
            }
        }
    }

    Ok(())
}

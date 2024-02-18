use std::{collections::HashMap, sync::OnceLock};

use anyhow::{anyhow, Ok, Result};
use reqwest::blocking::Client;
use serde_json::json;

const TOKEN_URL: &str = "https://auth.aliyundrive.com/v2/account/token";
const SIGN_URL: &str = "https://member.aliyundrive.com/v1/activity/sign_in_list";
const SIGN_REWARD_URL: &str = "https://member.aliyundrive.com/v1/activity/sign_in_reward";

pub fn get_client() -> &'static Client {
    static INSTANCE: OnceLock<Client> = OnceLock::new();
    INSTANCE.get_or_init(Client::new)
}

fn aliyun_sign(access_token: &str, map: HashMap<&str, &str>) -> Result<i64> {
    let client = get_client();
    let sign_response = client
        .post(SIGN_URL)
        .bearer_auth(access_token)
        .json(&map)
        .send()?;

    if !sign_response.status().is_success() {
        return Err(anyhow!("sign failed, message:{}", sign_response.text()?));
    }

    let sign_body = sign_response.json::<serde_json::Value>()?;
    let sign_result = sign_body["success"].as_bool().unwrap_or(false);
    if !sign_result {
        return Err(anyhow!("sign failed"));
    }

    let sign_count = sign_body["result"]["signInCount"].as_i64().unwrap_or(0);
    Ok(sign_count)
}

fn aliyun_reward(access_token: &str, sign_count: i64) -> Result<()> {
    let client = get_client();
    let response = client
        .post(SIGN_REWARD_URL)
        // .header("Authorization", format!("Bearer {access_token}"))
        .bearer_auth(access_token)
        .json(&json!({
            "signInDay":sign_count
        }))
        .send()?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "aliyun_reward response failed, message:{}",
            response.text()?
        ));
    }

    let body = response.json::<serde_json::Value>()?;
    let result = body["success"].as_bool().unwrap_or(false);
    if !result {
        return Err(anyhow!("reward failed, message:{}", body));
    }
    println!("reward success");
    Ok(())
}

pub fn aliyun_auto_sign(token: &str) -> Result<()> {
    let client = get_client();

    let mut map = HashMap::with_capacity(2);
    map.insert("grant_type", "refresh_token");
    map.insert("refresh_token", token);

    let token_response = client.post(TOKEN_URL).json(&map).send()?;

    if !token_response.status().is_success() {
        return Err(anyhow!(
            "token check failed, message:{}",
            token_response.text()?
        ));
    }

    let token_data = token_response.json::<serde_json::Value>()?;

    let access_token = token_data["access_token"].as_str();

    match access_token {
        Some(t) => {
            let sign_count = aliyun_sign(t, map)?;
            println!("count:{}", sign_count);
            if sign_count > 0 {
                return aliyun_reward(t, sign_count);
            }
            Ok(())
        }
        None => Err(anyhow!("get access token failed")),
    }
}

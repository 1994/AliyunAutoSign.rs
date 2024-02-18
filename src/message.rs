use anyhow::{anyhow, Ok};

use crate::sign::get_client;

pub fn send_message(chat_id: &str, token: &str, message: &str) -> anyhow::Result<()> {
    let client = get_client();
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");

    let params = [("chat_id", chat_id), ("text", message)];
    let response = client.post(url).form(&params).send()?;

    if !response.status().is_success() {
        return Err(anyhow!("send message failed, text:{}", response.text()?));
    }

    Ok(())
}

#[cfg(test)]
mod send_message_test {
    use super::send_message;

    #[test]
    fn test_send() {
        let r = send_message(
            "",
            "",
            "",
        );
        assert!(r.is_ok());
    }
}

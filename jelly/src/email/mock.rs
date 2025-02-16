use std::collections::HashMap;
use std::env::var;
use std::fmt;

use anyhow::anyhow;
use chrono::Utc;
use fancy_regex::Regex;
use serde_json;
use uuid::Uuid;

use super::common::{env_exists_and_not_empty, Email};

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    ["EMAIL_DEFAULT_FROM"]
        .iter()
        .for_each(|env| env_exists_and_not_empty(env));
}

struct MockResponse {
    /// The status code of the response, eg. 404.
    status_code: i32,
    /// The reason phrase of the response, eg. "Not Found".
    reason_phrase: String,
    /// The headers of the response. The header field names (the
    /// keys) are all lowercase.
    #[allow(dead_code)]
    headers: HashMap<String, String>,
    body: serde_json::Value,
}

impl fmt::Display for MockResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = self.body.get("Message").unwrap().as_str().unwrap();
        f.write_str(message)
    }
}

fn create_response(
    status_code: i32,
    reason_phrase: &str,
    body: &serde_json::Value,
) -> MockResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    MockResponse {
        status_code,
        headers,
        reason_phrase: reason_phrase.into(),
        body: body.clone(),
    }
}

impl Email {
    /// Send the email. Relies on you ensuring that `EMAIL_DEFAULT_FROM`,
    /// is set in your `.env`.
    pub fn send_via_mock(&self) -> Result<(), anyhow::Error> {
        let pattern = var("EMAIL_MOCK_BOUNCE_PATTERN").unwrap_or_else(|_| "^$".to_string());
        let re = Regex::new(&pattern).unwrap();
        let resp = match re.find(&self.to) {
            Ok(_) => {
                debug!("Mocking hard bounce for mail to {}.", &self.to);
                create_response(
                    200,
                    "OK",
                    &serde_json::json!({
                        "To": self.to,
                        "SubmittedAt": Utc::now(),
                        "MessageID": Uuid::new_v4(),
                        "ErrorCode": 406_i32,
                        "Message": "Address is inactive."}),
                )
            }
            _ => create_response(
                200,
                "OK",
                &serde_json::json!({
                    "To": self.to,
                    "SubmittedAt": Utc::now(),
                    "MessageID": Uuid::new_v4(),
                    "ErrorCode": 0_i32,
                    "Message": "OK"}),
            ),
        };

        if resp.status_code == 200 {
            debug!("Mail sent to {} via mock.", &self.to);
            Ok(())
        } else {
            Err(anyhow!(
                "Sending mail to {} via mock failed. API call returns code {} : {} \n {} ",
                &self.to,
                resp.status_code,
                resp.reason_phrase,
                resp
            ))
        }
    }
}

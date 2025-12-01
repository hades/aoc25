use std::collections::HashMap;
use std::error;
use std::fmt;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::time::Duration;

use aho_corasick::AhoCorasick;
use http::HeaderValue;
use log::debug;
use regex::Regex;
use reqwest::Url;

/// Implements a CookieStore for the sole purpose of transmitting the Advent of Code
/// session cookie. Will not store any other cookies.
struct AocSessionCookieStore {
    // Needs to be Arc<RwLock<_>> because CookieStore must implement Send and Sync
    // https://docs.rs/reqwest/latest/reqwest/cookie/trait.CookieStore.html
    cookie: Arc<RwLock<Option<String>>>,
}

impl AocSessionCookieStore {
    fn new() -> AocSessionCookieStore {
        AocSessionCookieStore {
            cookie: Arc::new(None.into()),
        }
    }
}

impl reqwest::cookie::CookieStore for AocSessionCookieStore {
    fn set_cookies(&self, _cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, _url: &Url) {}
    fn cookies(&self, _url: &Url) -> Option<HeaderValue> {
        match &*self.cookie.read().unwrap() {
            None => {
                debug!("AoC cookie has not been set");
                None
            }
            Some(cookie) => match HeaderValue::from_str(format!("session={cookie}").as_str()) {
                Ok(hv) => Some(hv),
                Err(e) => {
                    debug!("failed to create HeaderValue from cookie string: {e}");
                    None
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error") // TODO
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::HttpError(ref e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::HttpError(e)
    }
}

pub struct AocClient {
    base_url: String,
    cookie_store: Arc<AocSessionCookieStore>,
    client: reqwest::blocking::Client,
}

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Accepted,
    Rejected,
    RejectedTooLow,
    RejectedTooHigh,
    Throttled(Duration),
}

fn parse_validation_response(text: &str) -> ValidationResult {
    static PATTERNS: &[&str; 4] = &[
        "You gave an answer too recently",
        "That's the right answer",
        "your answer is too high",
        "your answer is too low",
    ];
    static AC: LazyLock<AhoCorasick> = LazyLock::new(|| {
        AhoCorasick::new(PATTERNS).expect("AhoCorasick automaton for parse_validation_response()")
    });
    static TIMEOUT_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"You have (?:(\d+)m )?(\d+)s left to wait")
            .expect("Regex compilation for parse_validation_response()")
    });
    let mut result = ValidationResult::Rejected;
    for mat in AC.find_iter(text) {
        result = match mat.pattern().as_usize() {
            0 => TIMEOUT_RE
                .captures(text)
                .map_or(ValidationResult::Rejected, |caps| {
                    let minutes = caps.get(1).map_or(0, |minutes| {
                        if minutes.is_empty() {
                            0
                        } else {
                            minutes
                                .as_str()
                                .parse::<u64>()
                                .expect("couldn't parse minutes")
                        }
                    });
                    let seconds = caps.get(2).expect("no group in parsed response");
                    let seconds = seconds
                        .as_str()
                        .parse::<u64>()
                        .expect("couldn't parse seconds");
                    ValidationResult::Throttled(Duration::from_secs(seconds + 60 * minutes))
                }),
            1 => ValidationResult::Accepted,
            2 => ValidationResult::RejectedTooHigh,
            3 => ValidationResult::RejectedTooLow,
            _ => panic!("did not expect pattern from AhoCorasick"),
        }
    }
    result
}

impl AocClient {
    pub fn new_with_base(base_url: &str) -> Result<AocClient, Error> {
        // We need to use an Arc here because reqwest::ClientBuilder requires an
        // Arc<C> of CookieStore:
        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html
        let cookie_store = Arc::new(AocSessionCookieStore::new());
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("aoc18 (https://github.com/hades/aoc25)")
            .cookie_provider(cookie_store.clone())
            .build()?;
        Ok(AocClient {
            base_url: String::from(base_url),
            cookie_store,
            client,
        })
    }

    pub fn new() -> Result<AocClient, Error> {
        Self::new_with_base("https://adventofcode.com/")
    }

    pub fn get_puzzle_input(&self, day: i8) -> Result<String, Error> {
        let url = self.base_url.clone() + format!("2025/day/{0}/input", day).as_str();
        let response = self.client.get(url).send()?;
        match response.error_for_status() {
            Ok(response) => Ok(response.text()?),
            Err(e) => Err(Error::HttpError(e)),
        }
    }

    pub fn submit_answer(
        &self,
        day: i8,
        level: i8,
        answer: &str,
    ) -> Result<ValidationResult, Error> {
        let url = self.base_url.clone() + format!("2025/day/{0}/answer", day).as_str();
        let request = self
            .client
            .post(url)
            .form(&HashMap::from([
                ("level", level.to_string()),
                ("answer", answer.to_string()),
            ]))
            .build()?;
        let response = self.client.execute(request)?;
        match response.error_for_status() {
            Ok(response) => Ok(parse_validation_response(response.text()?.as_str())),
            Err(e) => Err(Error::HttpError(e)),
        }
    }

    pub fn set_cookie(&self, cookie: &str) {
        *self.cookie_store.cookie.write().unwrap() = Some(String::from(cookie));
    }
}

#[cfg(test)]
mod tests;

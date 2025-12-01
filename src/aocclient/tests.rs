use httptest::Expectation;
use httptest::Server;
use httptest::ServerPool;
use httptest::matchers::all_of;
use httptest::matchers::contains;
use httptest::matchers::matches;
use httptest::matchers::request;
use httptest::matchers::url_decoded;
use httptest::responders::status_code;

use super::*;

static SERVER_POOL: ServerPool = ServerPool::new(2);

fn server_url(server: &Server) -> String {
    let url = server.url("/");
    let scheme = url.scheme().unwrap();
    let authority = url.authority().unwrap();
    format!("{scheme}://{authority}/")
}

#[test]
fn test_get_input_success() {
    let server = SERVER_POOL.get_server();
    let m = all_of![
        request::method("GET"),
        request::path(matches("/2025/day/19/input")),
        request::headers(contains(("cookie", "session=deadbeef"))),
    ];
    server.expect(Expectation::matching(m).respond_with(status_code(200)));
    let client =
        AocClient::new_with_base(server_url(&server).as_str()).expect("creating AoC client");
    client.set_cookie("deadbeef");
    assert_eq!(client.get_puzzle_input(19).expect("getting input"), "");
}

fn submit_answer_test(body: &'static str, expected_result: ValidationResult) {
    let server = SERVER_POOL.get_server();
    let m = all_of![
        request::method("POST"),
        request::path(matches("/2025/day/19/answer")),
        request::headers(contains(("cookie", "session=deadbeef"))),
        request::body(url_decoded(contains(("level", "1")))),
        request::body(url_decoded(contains(("answer", "THE ANSWER")))),
    ];
    server.expect(Expectation::matching(m).respond_with(status_code(200).body(body)));
    let client =
        AocClient::new_with_base(server_url(&server).as_str()).expect("creating AoC client");
    client.set_cookie("deadbeef");
    assert_eq!(
        client
            .submit_answer(19, 1, "THE ANSWER")
            .expect("getting input"),
        expected_result
    );
}

#[test]
fn test_submit_answer_success() {
    submit_answer_test(
        "<html><p>That\'s the right answer</p></html>",
        ValidationResult::Accepted,
    );
}

#[test]
fn test_submit_answer_rejected() {
    submit_answer_test(
        "<html><p>That\'s not the right answer</p></html>",
        ValidationResult::Rejected,
    );
}

#[test]
fn test_submit_answer_rejected_too_low() {
    submit_answer_test(
        "<html><p>That\'s not the right answer - your answer is too low.</p></html>",
        ValidationResult::RejectedTooLow,
    );
}

#[test]
fn test_submit_answer_rejected_too_high() {
    submit_answer_test(
        "<html><p>That\'s not the right answer - your answer is too high.</p></html>",
        ValidationResult::RejectedTooHigh,
    );
}

#[test]
fn test_submit_answer_throttled() {
    submit_answer_test(
        "<html><p>You gave an answer too recently. You have 57s left to wait</p></html>",
        ValidationResult::Throttled(Duration::from_secs(57)),
    );
}

#[test]
fn test_submit_answer_throttled_with_minutes() {
    submit_answer_test(
        "<html><p>You gave an answer too recently. You have 5m 31s left to wait</p></html>",
        ValidationResult::Throttled(Duration::from_secs(331)),
    );
}

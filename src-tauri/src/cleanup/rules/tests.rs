use super::*;
use std::time::Instant;

fn clean(text: &str) -> String {
    RuleCleaner.clean(text).unwrap()
}

#[test]
fn removes_pure_fillers() {
    assert_eq!(clean("I um went to the store"), "I went to the store");
    assert_eq!(clean("uh I think so"), "I think so");
    assert_eq!(clean("the erm thing is"), "The thing is");
}

#[test]
fn removes_multiple_fillers() {
    assert_eq!(
        clean("um so uh I was hmm thinking"),
        "I was thinking"
    );
}

#[test]
fn removes_contextual_like() {
    assert_eq!(
        clean("it was like going to happen"),
        "It was going to happen"
    );
    assert_eq!(
        clean("she is like really smart"),
        "She is really smart"
    );
}

#[test]
fn keeps_like_in_normal_usage() {
    assert_eq!(clean("I like cats"), "I like cats");
    assert_eq!(clean("it looks like rain"), "It looks like rain");
}

#[test]
fn removes_you_know_filler() {
    assert_eq!(
        clean("the thing is you know really good"),
        "The thing is really good"
    );
}

#[test]
fn protects_do_you_know() {
    assert_eq!(clean("do you know the answer"), "Do you know the answer");
    assert_eq!(
        clean("did you know that works"),
        "Did you know that works"
    );
}

#[test]
fn removes_so_at_sentence_start() {
    assert_eq!(clean("so I went home"), "I went home");
    assert_eq!(
        clean("it was great. so I stayed"),
        "It was great. I stayed"
    );
}

#[test]
fn keeps_so_mid_sentence() {
    assert_eq!(clean("I was so happy"), "I was so happy");
}

#[test]
fn removes_basically_at_start() {
    assert_eq!(
        clean("basically the idea is simple"),
        "The idea is simple"
    );
}

#[test]
fn removes_actually_at_start() {
    assert_eq!(clean("actually I disagree"), "I disagree");
}

#[test]
fn removes_i_mean_at_start() {
    assert_eq!(clean("i mean it could work"), "It could work");
}

#[test]
fn capitalizes_sentence_starts() {
    assert_eq!(clean("hello world"), "Hello world");
    assert_eq!(
        clean("hello. world is great"),
        "Hello. World is great"
    );
}

#[test]
fn capitalizes_pronoun_i() {
    assert_eq!(clean("i think i can do it"), "I think I can do it");
}

#[test]
fn normalizes_whitespace() {
    assert_eq!(clean("too   many   spaces"), "Too many spaces");
    assert_eq!(clean("  leading and trailing  "), "Leading and trailing");
}

#[test]
fn handles_empty_string() {
    assert_eq!(clean(""), "");
}

#[test]
fn combined_cleanup() {
    let input = "um so basically i was like going to the store you know and uh i think it was really good";
    let result = clean(input);
    // Should remove: um, so (start), basically (start), like (after "was"), you know, uh
    // Should capitalize: I, sentence start
    assert!(!result.contains("um"));
    assert!(!result.contains("uh"));
    assert!(!result.contains("you know"));
    assert!(result.starts_with('I'));
    assert!(result.contains('I'));
}

#[test]
fn latency_under_50ms() {
    let input = "um so basically i was like going to the um store you know and uh i think it was uh really um good actually i mean it was just like amazing";
    let start = Instant::now();
    for _ in 0..100 {
        let _ = clean(input);
    }
    let elapsed = start.elapsed();
    let avg_ms = elapsed.as_millis() as f64 / 100.0;
    assert!(
        avg_ms < 50.0,
        "Average cleanup took {:.1}ms, expected <50ms",
        avg_ms
    );
}

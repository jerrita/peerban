use lazy_static::lazy_static;
use regex::Regex;

const PT_KEYWORDS: [&str; 5] = ["?passkey=", "?authkey=", "?secure=", "?credential=", "private"];
lazy_static!(
    static ref RE: Regex = Regex::new(r"([a-zA-Z0-9]{32})").unwrap();
);

pub fn is_tracker_pt(tracker: &str) -> bool {
    let lower_tracker = tracker.to_lowercase();

    PT_KEYWORDS.iter().any(|&keyword| lower_tracker.contains(keyword))
        || RE.is_match(&lower_tracker)
}
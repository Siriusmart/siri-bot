use std::time::Duration;

const MINUTE: u64 = 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;

pub fn duration_string(duration: &Duration) -> String {
    let mut secs = duration.as_secs();

    let days = secs / DAY;
    secs -= days * DAY;

    let hours = secs / HOUR;
    secs -= hours * HOUR;

    let minutes = secs / MINUTE;
    secs -= minutes * MINUTE;

    let mut out = Vec::new();
    if days != 0 {
        out.push(format!("{}d", days));
    }

    if hours != 0 {
        out.push(format!("{}h", hours));
    }

    if minutes != 0 {
        out.push(format!("{}m", minutes));
    }

    if secs != 0 || !out.is_empty() {
        out.push(format!("{}s", secs));
    }

    out.join(" ")
}

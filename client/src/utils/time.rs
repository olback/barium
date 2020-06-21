const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;
const WEEK: u64 = 7 * DAY;

pub fn format_time(seconds: u64) -> String {

    let mut parts = Vec::<String>::new();

    let weeks = seconds / WEEK;
    let days = (seconds / DAY) % 7;
    let hours = (seconds / HOUR) % 24;
    let minutes = (seconds / MINUTE) % 60;

    let show_weeks = weeks > 0;
    if show_weeks {
        parts.push(format!("{}w", weeks));
    }

    let show_days = show_weeks || days > 0;
    if show_days {
        parts.push(format!("{}d", days));
    }

    let show_hours = show_days || hours > 0;
    if show_hours {
        parts.push(format!("{}h", hours));
    }

    let show_minutes = show_hours || minutes > 0;
    if show_minutes {
        parts.push(format!("{}m", minutes));
    }

    parts.join(" ")

}

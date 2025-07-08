use chrono::Local;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const GRAY: &str = "\x1b[90m";

pub fn logs(level: &str, args: &[&dyn std::fmt::Display]) {
    let now = Local::now();
    let timestamp = format!("[{}]", now.to_rfc2822());
    let colored_level = match level {
        "error" => format!("{}{}[ERROR]{}", RED, BOLD, RESET),
        "warn" => format!("{}{}[WARN]{}", YELLOW, BOLD, RESET),
        "info" => format!("{}{}[INFO]{}", BLUE, BOLD, RESET),
        _ => format!("{}{}[LOG]{}", GREEN, BOLD, RESET),
    };
    let mut msg = String::new();
    for arg in args {
        msg.push(' ');
        msg.push_str(&format!("{}", arg));
    }
    println!("{} {}{}{}{}", colored_level, GRAY, timestamp, RESET, msg);
}

pub fn logs_undefined(context: &str, err: &dyn std::fmt::Display) -> Option<()> {
    logs("error", &[&context, err]);
    None
}

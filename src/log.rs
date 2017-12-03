/// Log stuff to command line

pub enum Level {
    Success,
    Warning,
    Fail,
    Important,
}

fn log(s: &str, level: Level) {
    println!("[{}] {}", match level {
        Level::Success => "+",
        Level::Warning => ":",
        Level::Fail => "-",
        Level::Important => "!",
    }, s);
}

pub fn success(s: &str) {
    log(s, Level::Success);
}

pub fn warn(s: &str) {
    log(s, Level::Warning);
}

pub fn fail(s: &str) {
    log(s, Level::Fail);
}

pub fn important(s: &str) {
    log(s, Level::Important);
}
use std::time::Duration;
use regex::Regex;

pub struct Error {
    
}

pub enum Target {
    Stacktrace,
    Consolidate {
        duration: Duration,
        regexp: Regex,
        level: Option<Level>
    }
}

pub enum Level {

}

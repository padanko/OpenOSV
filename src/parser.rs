use regex::Regex;
use once_cell::sync::Lazy;

static BAN_COMMAND_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!aku:([0-9]+)").unwrap()  // Open2chと同じ !aku コマンド
});

pub fn aku_parse(text: String) {
    BAN_COMMAND_PATTERN.;
}


pub fn command_parse(text: String) {

}
use regex::Regex;
use crate::struct_defines::Thread;

fn is_decimal(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}
// BAN機能「aku」のコマンドを解析する
pub fn ban_parse(text: String, mut thread: Thread) -> Thread {
    let ban_command_pattern: Regex = Regex::new(r"!aku:([0-9]+)").unwrap();  // Open2chと同じ !aku コマンド
    match ban_command_pattern.captures(text.as_str()) {
        Some(val) => {
            for item in val.iter() {
                match item {
                    Some(item) => {
                        let item_str = item.as_str();
                        if is_decimal(item_str) {
                            let parsed_result: usize = item_str.parse().expect("数値へパースすることに失敗しました。");
                            if parsed_result <= thread.content.clone().len() { 
                                let ban_target_id: &String = &thread.content[parsed_result-1].id;
                                thread.banned.push(ban_target_id.clone());
                            }
                        }
                    }, None => {

                    }
                }
            }
        }, None => {
        }
    }
    thread

}


pub fn parse_commands(text: String, mut thread: Thread) -> Thread{

    thread = ban_parse(text, thread);


    return thread;
}
use regex::Regex;
use rand::{self, Rng};
use crate::struct_defines::Thread;

fn is_decimal(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn randint(a: i32, b:i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..b)
} 
// BAN機能「aku」のコマンドを解析する
pub fn ban_parse(mut text: String, mut thread: Thread) -> (Thread, String) {
    let ban_command_pattern: Regex = Regex::new(r"!aku:([0-9]+)").unwrap();  // Open2chと同じ !aku コマンド
    match ban_command_pattern.captures(text.clone().as_str()) {
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
                                text = format!("{}\n<b class='color-1'>■アク禁(BAN)  ID: {}</b>", &text, ban_target_id);
                            }
                        }
                    }, None => {

                    }
                }
            }
        }, None => {
        }
    }
    let results: (Thread, String) = (thread, text);
    return results;
}

// すべての「!random」をランダムな数字に置き換える
pub fn random_parse(mut text: String) -> String { 
    for _ in 0..text.as_str().matches("!random").count() {
        let replaced_str = format!("<b style='color-4'>{}</b>", randint(0, 100).to_string());
        text = text.replacen("!random", replaced_str.as_str(), 1)
    }
    text
}

pub fn parse_commands(mut text: String, thread: Thread) -> (Thread, String){

    text = random_parse(text);
    let (thread, text) = ban_parse(text, thread);
    let results: (Thread, String) = (thread, text);
    return results;

}
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
fn ban_parse(mut text: String, mut thread: Thread) -> (Thread, String) {
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
fn random_parse(mut text: String) -> String { 
    for _ in 0..text.as_str().matches("!random").count() {
        let replaced_str = format!("<b style='color-4'>{}</b>", randint(0, 100).to_string());
        text = text.replacen("!random", replaced_str.as_str(), 1)
    }
    text
}
 
// ココらへんは変数みたいな
fn setvar_parse(mut text: String, mut thread: Thread) -> (Thread, String) {
    let re = Regex::new(r"!set:([a-zA-Z0-9_]{1,32})=([0-9]{1,10})").unwrap();
    let captures: Vec<(String, String)> = re.captures_iter(&text).map(|cap| {
        (
            cap[1].to_string(),
            cap[2].to_string(),
        )
    }).collect();

    for (key, value) in captures.iter() {
        thread.var.insert(key.clone(), value.clone().parse().unwrap());
        text = format!("{}\n<b class='color-2'>変数セット {} = {}</b>", &text, key.clone(), value);
    }

    let results: (Thread, String) = (thread, text);
    return results;
}

fn getvar_parse(mut text: String, thread: Thread) -> String {
    let re = Regex::new(r"!get:([a-zA-Z0-9_]{1,32})").unwrap();
    let captures: Vec<String> = re.captures_iter(&text).map(|cap|  cap[1].to_string()).collect();

    for key in captures.iter() {
        let c = thread.var.get(key);
        match c
        {
            Some(value) => {
                text = format!("{}\n<b class='color-2'>変数セット {} = {}</b>", &text, key.clone(), value.to_string());
            },
            None => {
                
            }
        }
    }

    return text;
}



pub fn parse_commands(mut text: String, thread: Thread) -> (Thread, String){

    text = random_parse(text);
    let (thread, text) = ban_parse(text, thread.clone());
    let (thread, text) = setvar_parse(text, thread.clone());
    let text= getvar_parse(text, thread.clone());

    let results: (Thread, String) = (thread, text);
    return results;

}
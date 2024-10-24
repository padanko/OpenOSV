// Copyright (c) 2024 Honzawa yusei
// This software is licensed under the MIT License. See the LICENSE file for details.

// Rustの勉強に作った

// ウェブサーバー関係
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files as fs_actix;
use tera;

// シリアライズ系
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};

// ファイル関係
use std::{fs, io::Write};
use std::io::Read;

// 時間関係
use chrono::prelude::Local;

//ID生成に使う物
use sha2::{Sha256, Digest}; // ユーザーのID
use rand::Rng; // スレッドのID


//MOD

//ID生成の際に使う奴
const IDGEN_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+=/_";
const IDGEN_LENGTH: usize = 8; // 生成するIDの長さ

// IPアドレスを素にIDを生成する
fn generate_id(ipaddr: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ipaddr);
    let result = hasher.finalize();
    let id: String = result.iter().map(|&byte| {
        let index = (byte as usize) % IDGEN_CHARSET.len();
        IDGEN_CHARSET[index] as char
    }).take(IDGEN_LENGTH).collect();

    return id; // IDを返す
}

// IPアドレスを素にIDを生成する
fn generate_thread_id() -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..10).map(|_| rng.gen_range(0..10).to_string()).collect();
    return id; // IDを返す
}


// スレッドの一覧を取得する
fn thread_glob() -> Vec<Thread> {
    let mut thread_list: Vec<Thread> = Vec::new();
    match fs::read_dir("./BBS/") {
        Ok(r_dir) => {
            for entry in r_dir {
                let fname = entry.unwrap().file_name().to_str().unwrap().to_string();
                match fs::File::open(format!("./BBS/{}",fname)) {
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("エラー！ファイルの読み込みに失敗しました.");
                        let thr: Thread = from_str(&buf.as_str()).expect("エラー！それはJSONですか？");
                        thread_list.push(thr);
                    }, Err(_) => {
                        println!("エラー！ファイルの読み込みに失敗しました. ファイルは存在しますか？");
                    }
                }
            }
        }, Err(_) => {
            println!("エラー！BBSディレクトリは存在しますか？");

        }
    }
    return thread_list;
}

// JSON読み込んでThreadにする
fn to_thr_object(thrid: String) -> Result<Thread, String> {
    match fs::File::open(format!("./BBS/{}.json",thrid)) {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("エラー！ファイルの読み込みに失敗しました.");
            let thr: Thread = from_str(&buf.as_str()).expect("エラー！それはJSONですか？");
            Ok(thr)
        }, Err(_) => {
            Err(String::from("ファイルの読み込みに失敗しました."))
        }
    }
}

// レンダリングされる
fn replace_text(text: String) -> String {
    let mut text = text;
    text = text.replace("\n", "<br>");
    text
}

// リクエストパラメータの構造体
#[derive(Serialize, Deserialize, Debug)]
struct ThreadMakeParameters {
    title: String,
    name: String,
    text: String,
}

// レスのリクエストパラメータの構造体
#[derive(Serialize, Deserialize, Debug)]
struct ResponseMakeParameters {
    name: String,
    text: String,
    thrid: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Thread {
    id: String,
    len: i32,
    title: String,
    banned: Vec<String>,
    content: Vec<Response>,
    admin: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Response {
    name: String,
    text: String,
    date: String,
    id: String,
}

// インデックスページ
async fn page_index() -> impl Responder {
    let tera = tera::Tera::new("HTML/**/*").expect("Tera テンプレートエンジンの初期化に失敗しました.");
    let mut ctx = tera::Context::new();
    ctx.insert("thread_list", &thread_glob());
    let html = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

// 
async fn ev_make_thr(data: web::Form<ThreadMakeParameters>, req: HttpRequest) -> impl Responder {
    // ID生成に必要な情報
    let thr_id = generate_thread_id();
    match fs::File::create(format!("./BBS/{}.json", thr_id.clone())) {
        Ok(mut file) => {
            let remote_addr = req.peer_addr().map(|addr| addr.to_string()).unwrap_or("Unknown".to_string());

            let title = &data.title;
            let mut name = &data.name;
            let nanasi = "名無しさん".to_string();

            let text_ = &data.text;
            let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let id = generate_id(remote_addr);

            if name == "" {
                name = &nanasi;
            }

            let text = text_.clone().replace(">", "&gt;").replace("<", "&lt;");
            
            let inital_response: Response = Response {name: name.clone(), text: text, date: date, id: id.clone()};
            let thread_object: Thread = Thread {title: title.clone(), banned: vec![], content: vec![inital_response], admin: id, id: thr_id.clone(), len: 1};

            let buffer = to_string(&thread_object).unwrap();
            let _ = file.write_all(&buffer.as_bytes());
            HttpResponse::Ok().content_type("text/html").body(format!("<a href='/thread/{}' id='url'></a><script>location.href = document.getElementById('url').href</script>", thr_id.clone()))
        }, Err(_) => {
            HttpResponse::Ok().body("Error: The action could not be completed due to a glitch on the server side")
        }
    }
        
}

async fn page_thread(path: web::Path<(String,)>) -> impl Responder {
    let thrid = path.into_inner().0;
    match to_thr_object(thrid.clone()) {
        Ok(thr) => {
            let tera = tera::Tera::new("HTML/**/*").expect("Tera テンプレートエンジンの初期化に失敗しました.");
            let mut ctx = tera::Context::new();

            let mut contents: Vec<Response> = vec![]; 

            for item in thr.content {
                contents.push(Response { text: replace_text(item.text), name: item.name, date: item.date, id: item.id});
            }

            ctx.insert("thrid", &thrid);
            ctx.insert("title", &thr.title);
            ctx.insert("com", &contents);
            let html = tera.render("read.html", &ctx).unwrap();
            HttpResponse::Ok().content_type("text/html").body(html)    
        }, Err(_) => {
            HttpResponse::Ok().body("Error: Thread does not exist")
        }
    }
}

async fn ev_poll(path: web::Path<(String,)>) -> impl Responder {
    let thrid = path.into_inner().0;
    match to_thr_object(thrid.clone()) {
        Ok(mut old_thread) => {
            loop {
                let new_thread = to_thr_object(thrid.clone()).expect("エラー！ファイルは存在しますか？");

                if new_thread.clone().content != old_thread.content {
                    let newest_rsp__ = new_thread.content[new_thread.content.len() - 1].clone();
                    let text = replace_text(new_thread.content[new_thread.content.len() - 1].text.clone());

                    let newest_rsp = Response {text: text, name: newest_rsp__.name, date: newest_rsp__.date, id: newest_rsp__.id};

                    match serde_json::to_string(&newest_rsp) {
                        Ok(result) => {
                            return HttpResponse::Ok().content_type("application/json").body(result);
                        }, Err(_) => {
                            return HttpResponse::Ok().body("Error: Internal Server Error");
                        }
                    }
                }
            
                old_thread = new_thread;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }, Err(_) => {
            HttpResponse::Ok().body("Error: Thread does not exist")
        }
    }
}

async fn ev_make_rsp(data: web::Form<ResponseMakeParameters>, req: HttpRequest) -> impl Responder {
    let thrid = &data.0.thrid;
    match to_thr_object(thrid.clone()) {
        Ok(mut thr) => {
            match fs::File::create(format!("./BBS/{}.json", thrid)) {
                Ok(mut file) => {
                    let text_ = &data.0.text;
                    let mut name = &data.0.name;
                    let nanasi = "名無しさん".to_string();

                    if name == "" {
                        name = &nanasi;
                    }

                    let text = text_.clone().replace(">", "&gt;").replace("<", "&lt;");
        
                    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let remote_addr = req.peer_addr().map(|addr| addr.to_string()).unwrap_or("Unknown".to_string());
                    let id = generate_id(remote_addr);
                    
                    

                    thr.content.push(Response {name: name.clone(), text: text, date: date, id: id});
                    let buffer = to_string(&thr).unwrap();
                    let _ = file.write_all(buffer.as_bytes());
                    HttpResponse::Ok().body("SUC")
                }, Err(_) => {
                    HttpResponse::Ok().body("ERR2")
                }
            }


        }, Err(_) => {
            HttpResponse::Ok().body("ERR1")
        }
    }


}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(page_index))
        .route("/Post/Thread", web::post().to(ev_make_thr))
        .route("/Post/Response", web::post().to(ev_make_rsp))
        .route("/thread/{thrID}", web::get().to(page_thread)) 
        .route("/poll/{thrID}", web::get().to(ev_poll)) 
        .service(fs_actix::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
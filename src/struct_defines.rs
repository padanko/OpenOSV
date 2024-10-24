use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate)  struct Thread {
    pub id: String,
    pub len: i32,
    pub title: String,
    pub banned: Vec<String>,
    pub content: Vec<Response>,
    pub admin: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate)  struct Response {
    pub name: String,
    pub text: String,
    pub date: String,
    pub id: String,
}
use serde::Deserialize;
use chrono::{DateTime, FixedOffset, Utc, Datelike};

#[derive(Debug, Deserialize)]
struct ApiOutput {
    success : bool,
    data : Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    group : String,
    year : String,
    tp : String,
    date : String,
    courses : Vec<Course>,
}

#[derive(Debug, Deserialize)]
struct Course {
    id : i32,
    title: String,
    startTime : String,
    endTime : String,
    room : String,
    teacher : String,
    r#type : String,
}

fn main() {
    let response = reqwest::blocking::get("https://iut-room-viewer.gamo.one/api/v1/schedule?group=G4&tp=A")
        .expect("Erreur requête HTTP");
    let api_output: ApiOutput = response.json().expect("Erreur JSON");
    let group_color = "#BF616A";

    if api_output.success {
        let data = api_output.data;
        let group_format = format!("{}{} - {}", data.group, data.tp, data.year);                    
        println!("<span foreground='{}'>{}</span>",group_color ,group_format);
    }
}
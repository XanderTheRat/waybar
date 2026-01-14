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

    if api_output.success {
        let data = api_output.data;
        let mut courses = data.courses;
        let paris_offset = FixedOffset::east_opt(3600).unwrap();
        let now_utc = Utc::now();
        let now_paris = now_utc.with_timezone(&paris_offset);
        let mut course_found = false;

        let edt_color = "#E06C75";
        let no_course_color = "#98C379";

        courses.sort_by(|a, b| a.startTime.cmp(&b.startTime));

        for course in &courses {
            let start_utc: DateTime<Utc> = course.startTime.parse().expect("Erreur date start");
            let end_utc: DateTime<Utc> = course.endTime.parse().expect("Erreur date end");

            if end_utc > now_utc {
                let start_paris = start_utc.with_timezone(&paris_offset);
                if start_paris.date_naive() == now_paris.date_naive() {
                    let end_paris = end_utc.with_timezone(&paris_offset);
                    let start_hours = start_paris.format("%H:%M");
                    let end_hours = end_paris.format("%H:%M");
                    let status_prefix = if start_utc > now_utc {
                        "Prochain cours"
                    } else {
                        "Cours actuel"  
                    };
                    let hours_format = format!("{} : {} - {}", status_prefix, start_hours, end_hours);                    
                    println!("<span foreground='{}'>{}</span>",edt_color ,hours_format);                    
                    course_found = true;
                } else {
                    println!("<span foreground='{}'>Vous n'avez pas cours</span>", no_course_color);
                    course_found = true; 
                }
                break;
            }
        }
        if !course_found {
            println!("<span foreground='{}'>Vous n'avez pas cours</span>", no_course_color);
        }
    }
}
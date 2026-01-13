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
    let response = reqwest::blocking::get("http://panel.gamo.one:50006/api/v1/schedule?group=G4&tp=A")
        .expect("Erreur requête HTTP");
    let api_output: ApiOutput = response.json().expect("Erreur JSON");

    if api_output.success {
        let data = api_output.data;
        let mut courses = data.courses;
        let paris_offset = FixedOffset::east_opt(3600).unwrap();
        let now_utc = Utc::now();
        let now_paris = now_utc.with_timezone(&paris_offset);
        let mut course_found = false;

        courses.sort_by(|a, b| a.startTime.cmp(&b.startTime));
        
        for course in &courses {
            let end_utc: DateTime<Utc> = course.endTime.parse().expect("Erreur date end");

            if end_utc > now_utc {
                let class_type_format = format!("{} avec {} en {}", course.r#type, course.teacher, course.room);                    
                println!("{}", class_type_format);                    
                course_found = true;
            } else {
                println!("Vous n'avez pas cours");
                course_found = true; 
                }
                break;
            }
        if !course_found {
            println!("Vous n'avez pas cours");
        }
    }
}
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
        let mut course_found = false;
        let class_color = "#F38BA8";
        let no_course_color = "#A6E3A1";

        courses.sort_by(|a, b| a.startTime.cmp(&b.startTime));
        
        for course in &courses {
            let end_utc: DateTime<Utc> = course.endTime.parse().expect("");

            if end_utc > now_utc {
                let class_type_format = format!("<span foreground='{}'>{} avec {} en {}</span>",class_color ,course.r#type, course.teacher, course.room);                    
                println!("{}", class_type_format);                    
                course_found = true;
            } else {
                println!("<span foreground='{}'>Vous n'avez pas cours</span>", no_course_color);
                course_found = true; 
                }
                break;
            }
        if !course_found {
            println!("<span foreground='{}'>Vous n'avez pas cours</span>",no_course_color);
        }
    }
}
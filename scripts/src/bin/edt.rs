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
            let start_utc: DateTime<Utc> = course.startTime.parse().expect("Erreur date start");
            let end_utc: DateTime<Utc> = course.endTime.parse().expect("Erreur date end");

            if end_utc > now_utc {
                
                let start_paris = start_utc.with_timezone(&paris_offset);

                if start_paris.date_naive() == now_paris.date_naive() {
                    
                    let end_paris = end_utc.with_timezone(&paris_offset);
                    let start_hours = start_paris.format("%H:%M");
                    let end_hours = end_paris.format("%H:%M");
                    let group_format = format!("Vous êtes en {}{} - {}", data.group, data.tp, data.year);
                    let class_type_format = format!("{} avec {} en {}", course.r#type, course.teacher, course.room);
                    let status_prefix = if start_utc > now_utc {
                        "Prochain cours"
                    } else {
                        "Cours actuel"  
                    };
                    let hours_format = format!("{} - Début : {} | Fin : {}", status_prefix, start_hours, end_hours);
                    
                    println!("{}\n{}\n{}", group_format, class_type_format, hours_format);
                    
                    course_found = true;
                } else {
                    println!("Vous n'avez pas cours");
                    course_found = true; 
                }
                break;
            }
        }
        if !course_found {
            println!("Vous n'avez pas cours");
        }
    }
}
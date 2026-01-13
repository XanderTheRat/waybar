use serde::Deserialize;
use chrono::{DateTime, FixedOffset, Utc};

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

fn format_hours(time : String) -> String {
    let utc_date: DateTime<Utc> = time.parse().expect("Erreur de parsing");
    let offset = FixedOffset::east_opt(3600).expect("Erreur d'offset");
    let date_plus_one = utc_date.with_timezone(&offset);
    let heure_seule = date_plus_one.format("%H:%M").to_string();
    heure_seule
}

fn main() {
    let response = reqwest::blocking::get("http://panel.gamo.one:50006/api/v1/schedule?group=G4&tp=A").unwrap();
    let api_output: ApiOutput = response.json().unwrap();
    println!("{:?}", api_output);
    let success = api_output.success;
    if success {
    	let data: Data = api_output.data;
    	let group = data.group;
    	let year = data.year;
    	let tp = data.tp;
    	let date = data.date;
    	let courses = data.courses;
    	println!("{}", date);

    	for course in &courses {
    		let start = &course.startTime;
    		let end = &course.endTime;
    		let room = &course.room;
    		let teacher = &course.teacher;
    		let course_type = &course.r#type;

    		let start_hours = format_hours(start.to_string());
    		let end_hours = format_hours(end.to_string());

    		let group_format = format!("Vous êtes en {}{}", group, tp);
    		let class_type_format = format!("{} avec {} en {}", course_type,teacher, room);
    		let hours_format = format!("Début : {} | Fin : {}", start_hours, end_hours);
			println!("{} \n{}\n{}\n", group_format, class_type_format, hours_format);
    	}
    }
}
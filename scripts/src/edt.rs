use chrono::{DateTime, Local, Utc};
use serde::Deserialize;

// Rep API
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub data: ScheduleData,
}

// Donnees
#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleData {
    pub group: String,
    pub year: String,
    pub tp: String,
    pub date: String,
    pub courses: Vec<Course>,
}

impl ScheduleData {
    pub fn format_group_info(&self, color: &str) -> String {
        format!(
            "<span foreground='{}'>{}{} - {}</span>",
            color, self.group, self.tp, self.year
        )
    }

    pub fn active_or_next_course(&self) -> Option<&Course> { // Get prochain cours
        let mut sorted_courses: Vec<&Course> = self.courses.iter().collect();
        sorted_courses.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        let now_utc = Utc::now();
        let now_local = Local::now();

        for course in sorted_courses {
            if let (Ok(start_utc), Ok(end_utc)) = (course.start_utc(), course.end_utc()) {
                if end_utc > now_utc {
                    let start_local = start_utc.with_timezone(&Local);
                    if start_local.date_naive() == now_local.date_naive() {
                        return Some(course);
                    }
                    return None;
                }
            }
        }

        None
    }

    pub fn format_edt(&self, edt_color: &str, no_course_color: &str) -> String {
        match self.active_or_next_course() {
            Some(course) => course.format_time_status(edt_color),
            None => format!("<span foreground='{}'>Aucun cours</span>", no_course_color),
        }
    }

    pub fn format_room(&self, class_color: &str, no_course_color: &str) -> String {
        match self.active_or_next_course() {
            Some(course) => course.format_room_info(class_color),
            None => format!("<span foreground='{}'>Aucun cours</span>", no_course_color),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Course {
    pub id: i32,
    #[serde(default)]
    pub code: Option<String>,
    pub title: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub room: String,
    pub teacher: String,
    #[serde(rename = "type")]
    pub course_type: String,
}

impl Course {
    pub fn start_utc(&self) -> Result<DateTime<Utc>, chrono::ParseError> {
        self.start_time.parse::<DateTime<Utc>>()
    }

    pub fn end_utc(&self) -> Result<DateTime<Utc>, chrono::ParseError> {
        self.end_time.parse::<DateTime<Utc>>()
    }

    pub fn start_local(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        self.start_utc().map(|utc| utc.with_timezone(&Local))
    }

    pub fn end_local(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        self.end_utc().map(|utc| utc.with_timezone(&Local))
    }

    pub fn format_room_info(&self, color: &str) -> String {
        format!(
            "<span foreground='{}'>{} avec {} en {}</span>",
            color, self.course_type, self.teacher, self.room
        )
    }

    pub fn format_time_status(&self, color: &str) -> String {
        let now_utc = Utc::now();
        let start_utc = self.start_utc().unwrap_or(now_utc);
        let start_local = self.start_local().unwrap_or_else(|_| Local::now());
        let end_local = self.end_local().unwrap_or_else(|_| Local::now());

        let start_hours = start_local.format("%H:%M");
        let end_hours = end_local.format("%H:%M");

        let status_text = if start_utc > now_utc {
            format!("A suivre : {} - {}", start_hours, end_hours)
        } else {
            format!("Fin : {}", end_hours)
        };

        format!("<span foreground='{}'>{}</span>", color, status_text)
    }
}

pub struct Edt {
    pub td_group: String,
    pub tp_group: String,
}

impl Edt {
    pub const DEFAULT_TD_GROUP: &'static str = "G7";
    pub const DEFAULT_TP_GROUP: &'static str = "B";

    pub fn new(td_group: impl Into<String>, tp_group: impl Into<String>) -> Self {
        Self {
            td_group: td_group.into(),
            tp_group: tp_group.into(),
        }
    }

    pub fn default_group() -> Self {
        Self::new(Self::DEFAULT_TD_GROUP, Self::DEFAULT_TP_GROUP)
    }

    pub fn fetch(&self) -> Result<ScheduleData, Box<dyn std::error::Error>> {
        let url = format!(
            "https://iut-room-viewer.gamo.one/api/v1/schedule?group={}&tp={}",
            self.td_group, self.tp_group
        );
        let response = reqwest::blocking::get(&url)?;
        let api_output: ApiResponse = response.json()?;

        if api_output.success {
            Ok(api_output.data)
        } else {
            Err("Échec de la réponse de l'API (success = false)".into())
        }
    }
}

impl Default for Edt {
    fn default() -> Self {
        Self::default_group()
    }
}

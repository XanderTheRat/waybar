use crate::edt::{Course, Edt, ScheduleData};
use crate::lock::theme::ColorTheme;
use chrono::Utc;

pub struct CardContent {
    pub badge: String,
    pub theme: ColorTheme,
    pub title: String,
    pub time_and_room: String,
    pub group_info: String,
}

impl CardContent {
    pub fn from_schedule_result(res: Result<ScheduleData, Box<dyn std::error::Error>>) -> Self {
        match res {
            Ok(schedule) => Self::from_schedule(&schedule),
            Err(_) => Self::fallback_offline(),
        }
    }

    fn from_schedule(schedule: &ScheduleData) -> Self {
        let base_group = format!("Groupe {}{} • {}", schedule.group, schedule.tp, schedule.year);
        if let Some(course) = schedule.active_or_next_course() {
            Self::from_course(course, base_group)
        } else {
            Self {
                badge: "AUJOURD'HUI".to_string(),
                theme: ColorTheme::for_course_type("SAE"),
                title: "Aucun cours à venir".to_string(),
                time_and_room: "Fin des cours pour aujourd'hui".to_string(),
                group_info: base_group,
            }
        }
    }

    fn from_course(course: &Course, base_group: String) -> Self {
        let now_utc = Utc::now();
        let start_utc = course.start_utc().unwrap_or(now_utc);
        let is_upcoming = start_utc > now_utc;

        let badge = if is_upcoming {
            format!("PROCHAIN COURS ({})", course.course_type)
        } else {
            format!("COURS EN COURS ({})", course.course_type)
        };

        let theme = ColorTheme::for_course_type(&course.course_type);
        let title = course.title.clone();

        let start_str = course
            .start_local()
            .map(|d| d.format("%H:%M").to_string())
            .unwrap_or_else(|_| "--:--".to_string());
        let end_str = course
            .end_local()
            .map(|d| d.format("%H:%M").to_string())
            .unwrap_or_else(|_| "--:--".to_string());
        let duration = course.duration_formatted();

        let mut time_and_room = format!("{} - {} ({})", start_str, end_str, duration);
        if !course.room.is_empty() {
            time_and_room.push_str(&format!("  •  Salle {}", course.room));
        }

        let mut group_info = base_group;
        if !course.teacher.is_empty() {
            group_info.push_str(&format!("  •  {}", course.teacher));
        }

        Self {
            badge,
            theme,
            title,
            time_and_room,
            group_info,
        }
    }

    fn fallback_offline() -> Self {
        Self {
            badge: "EMPLOI DU TEMPS".to_string(),
            theme: ColorTheme::for_course_type("SAE"),
            title: "Emploi du temps indisponible".to_string(),
            time_and_room: "Planning hors ligne".to_string(),
            group_info: format!("Groupe {}{} • BUT3", Edt::DEFAULT_TD_GROUP, Edt::DEFAULT_TP_GROUP),
        }
    }
}

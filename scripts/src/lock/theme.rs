#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub card_bg_start: String,
    pub card_bg_end: String,
    pub card_border: String,
    pub badge_bg: String,
    pub badge_border: String,
    pub badge_color: String,
}

impl ColorTheme {
    pub fn for_course_type(course_type: &str) -> Self {
        let colors = crate::Colors::load();
        let ct_upper = course_type.trim().to_uppercase();
        let prefix = if ct_upper.contains("CM") {
            "lock_cm"
        } else if ct_upper.contains("TD") {
            "lock_td"
        } else if ct_upper.contains("TP") {
            "lock_tp"
        } else {
            "lock_default"
        };

        Self {
            card_bg_start: colors.get(&format!("{prefix}_card_bg_start")),
            card_bg_end: colors.get(&format!("{prefix}_card_bg_end")),
            card_border: colors.get(&format!("{prefix}_card_border")),
            badge_bg: colors.get(&format!("{prefix}_badge_bg")),
            badge_border: colors.get(&format!("{prefix}_badge_border")),
            badge_color: colors.get(&format!("{prefix}_badge_color")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorTheme {
    pub card_bg_start: &'static str,
    pub card_bg_end: &'static str,
    pub card_border: &'static str,
    pub badge_bg: &'static str,
    pub badge_border: &'static str,
    pub badge_color: &'static str,
}

impl ColorTheme {
    pub fn for_course_type(course_type: &str) -> Self {
        let ct_upper = course_type.trim().to_uppercase();
        if ct_upper.contains("CM") {
            Self {
                card_bg_start: "#231e0f",
                card_bg_end: "#362c12",
                card_border: "#fff",
                badge_bg: "rgba(245, 158, 11, 0.22)",
                badge_border: "rgba(245, 158, 11, 0.55)",
                badge_color: "#FBBF24",
            }
        } else if ct_upper.contains("TD") {
            Self {
                card_bg_start: "#24101e",
                card_bg_end: "#38162f",
                card_border: "rgba(244, 114, 182, 0.45)",
                badge_bg: "rgba(236, 72, 153, 0.22)",
                badge_border: "rgba(236, 72, 153, 0.55)",
                badge_color: "#F472B6",
            }
        } else if ct_upper.contains("TP") {
            Self {
                card_bg_start: "#0d1b2a",
                card_bg_end: "#152c42",
                card_border: "rgba(56, 189, 248, 0.45)",
                badge_bg: "rgba(14, 165, 233, 0.22)",
                badge_border: "rgba(14, 165, 233, 0.55)",
                badge_color: "#38BDF8",
            }
        } else {
            Self {
                card_bg_start: "#0d2318",
                card_bg_end: "#143725",
                card_border: "rgba(74, 222, 128, 0.45)",
                badge_bg: "rgba(16, 185, 129, 0.22)",
                badge_border: "rgba(16, 185, 129, 0.55)",
                badge_color: "#4ADE80",
            }
        }
    }
}

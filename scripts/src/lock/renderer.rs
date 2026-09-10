use crate::lock::content::CardContent;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub struct SvgCardRenderer {
    pub center_x: i32,
    pub top_y: i32,
    pub width: i32,
    pub height: i32,
    pub title_color: String,
    pub time_color: String,
    pub group_color: String,
    pub sep_color: String,
    pub shadow_color: String,
}

impl Default for SvgCardRenderer {
    fn default() -> Self {
        let colors = crate::Colors::load();
        Self {
            center_x: 1275,
            top_y: 175,
            width: 580,
            height: 210,
            title_color: colors.get("lock_card_title"),
            time_color: colors.get("lock_card_time"),
            group_color: colors.get("lock_card_group"),
            sep_color: colors.get("lock_card_separator"),
            shadow_color: colors.get("lock_card_shadow"),
        }
    }
}

impl SvgCardRenderer {
    pub fn new(center_x: i32, top_y: i32, width: i32, height: i32) -> Self {
        let colors = crate::Colors::load();
        Self {
            center_x,
            top_y,
            width,
            height,
            title_color: colors.get("lock_card_title"),
            time_color: colors.get("lock_card_time"),
            group_color: colors.get("lock_card_group"),
            sep_color: colors.get("lock_card_separator"),
            shadow_color: colors.get("lock_card_shadow"),
        }
    }

    pub fn render(&self, content: &CardContent) -> String {
        let esc_badge = escape_xml(&content.badge);
        let esc_title = escape_xml(&content.title);
        let esc_time = escape_xml(&content.time_and_room);
        let esc_group = escape_xml(&content.group_info);

        let title_len = content.title.chars().count();
        let title_font_size = if title_len > 38 {
            18
        } else if title_len > 26 {
            21
        } else {
            24
        };

        let card_x = self.center_x - self.width / 2;
        let card_y = self.top_y;

        let badge_width = (esc_badge.chars().count() * 8 + 36).max(120);
        let badge_x = self.center_x - (badge_width as i32 / 2);
        let badge_y = card_y + 20;

        let sep_x1 = card_x + 35;
        let sep_x2 = card_x + self.width - 35;
        let sep_y = card_y + 148;

        format!(
            r###"<svg width="1920" height="1080" viewBox="0 0 1920 1080" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="edtCardGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{card_bg_start}" stop-opacity="0.75"/>
      <stop offset="100%" stop-color="{card_bg_end}" stop-opacity="0.65"/>
    </linearGradient>
    <filter id="cardShadow" x="-10%" y="-10%" width="120%" height="130%">
      <feDropShadow dx="0" dy="10" stdDeviation="18" flood-color="{shadow_color}" flood-opacity="0.6"/>
    </filter>
  </defs>

  <!-- edt -->
  <g filter="url(#cardShadow)">
    <rect x="{card_x}" y="{card_y}" width="{card_width}" height="{card_height}" rx="5" ry="5"
          fill="url(#edtCardGrad)"
          stroke="{card_border}"
          stroke-width="1.5" />
  </g>

  <rect x="{badge_x}" y="{badge_y}" width="{badge_width}" height="24" rx="15" ry="15"
        fill="{badge_bg}"
        stroke="{badge_border}"
        stroke-width="1" />
  <text x="{card_cx}" y="{badge_text_y}" font-family="Noto Sans, sans-serif" font-size="11" font-weight="700" fill="{badge_color}" text-anchor="middle" letter-spacing="1.5">
    {esc_badge}
  </text>

  <!-- prochn crs -->
  <text x="{card_cx}" y="{title1_y}" font-family="Noto Sans, sans-serif" font-size="{title1_font_size}" font-weight="700" fill="{title_color}" text-anchor="middle">
    {esc_title}
  </text>

  <!-- horaires -->
  <text x="{card_cx}" y="{title2_y}" font-family="Noto Sans, sans-serif" font-size="16" font-weight="500" fill="{time_color}" text-anchor="middle">
    {esc_time}
  </text>

  <line x1="{sep_x1}" y1="{sep_y}" x2="{sep_x2}" y2="{sep_y}" stroke="{sep_color}" stroke-width="1" />

  <text x="{card_cx}" y="{group_y}" font-family="Noto Sans, sans-serif" font-size="13" font-weight="400" fill="{group_color}" text-anchor="middle" letter-spacing="0.5">
    {esc_group}
  </text>
</svg>"###,
            card_bg_start = content.theme.card_bg_start,
            card_bg_end = content.theme.card_bg_end,
            card_border = content.theme.card_border,
            badge_bg = content.theme.badge_bg,
            badge_border = content.theme.badge_border,
            badge_color = content.theme.badge_color,
            shadow_color = self.shadow_color,
            title_color = self.title_color,
            time_color = self.time_color,
            sep_color = self.sep_color,
            group_color = self.group_color,
            card_x = card_x,
            card_y = card_y,
            card_width = self.width,
            card_height = self.height,
            card_cx = self.center_x,
            badge_x = badge_x,
            badge_y = badge_y,
            badge_width = badge_width,
            badge_text_y = badge_y + 16,
            title1_font_size = title_font_size,
            title1_y = card_y + 78,
            title2_y = card_y + 118,
            sep_x1 = sep_x1,
            sep_x2 = sep_x2,
            sep_y = sep_y,
            group_y = card_y + 180,
            esc_badge = esc_badge,
            esc_title = esc_title,
            esc_time = esc_time,
            esc_group = esc_group,
        )
    }
}

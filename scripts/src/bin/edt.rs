use waybar::edt::Edt;

fn main() {
    let colors = waybar::Colors::load();
    let edt = Edt::default();
    let color_edt = colors.get("color_edt");
    let color_no_course = colors.get("color_edt_no_course");
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_edt(&color_edt, &color_no_course));
        }
        Err(_) => {
            println!("<span foreground='{}'>Aucun cours</span>", color_no_course);
        }
    }
}
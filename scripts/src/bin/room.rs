use waybar::edt::Edt;

fn main() {
    let colors = waybar::Colors::load();
    let edt = Edt::default();
    let color_room = colors.get("color_room");
    let color_no_course = colors.get("color_room_no_course");
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_room(&color_room, &color_no_course));
        }
        Err(_) => {
            println!("<span foreground='{}'>Aucun cours</span>", color_no_course);
        }
    }
}
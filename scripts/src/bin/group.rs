use waybar::edt::Edt;

fn main() {
    let colors = waybar::Colors::load();
    let edt = Edt::default();
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_group_info(&colors.get("color_group")));
        }
        Err(e) => eprintln!("Erreur EDT: {e}"),
    }
}
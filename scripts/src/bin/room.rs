use waybar::edt::Edt;

fn main() {
    let edt = Edt::default();
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_room("#F38BA8", "#A6E3A1"));
        }
        Err(_) => {
            println!("<span foreground='#A6E3A1'>Aucun cours</span>");
        }
    }
}
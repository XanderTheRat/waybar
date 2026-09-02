use waybar::edt::Edt;

fn main() {
    let edt = Edt::default();
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_edt("#E06C75", "#98C379"));
        }
        Err(_) => {
            println!("<span foreground='#98C379'>Aucun cours</span>");
        }
    }
}
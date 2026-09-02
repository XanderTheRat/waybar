use waybar::edt::Edt;

fn main() {
    let edt = Edt::default();
    match edt.fetch() {
        Ok(schedule) => {
            println!("{}", schedule.format_group_info("#BF616A"));
        }
        Err(e) => eprintln!("Erreur EDT: {e}"),
    }
}
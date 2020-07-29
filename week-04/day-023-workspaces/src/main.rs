use vgebrev_first_crate::arts;
use vgebrev_first_crate::arts::PrimaryColor;
use vgebrev_first_crate::arts::SecondaryColor;

fn main() {
    let secondary = arts::mix(PrimaryColor::Yellow, PrimaryColor::Red);
    let color = get_color_string(secondary);
    println!("{}", color);
}

fn get_color_string(c: SecondaryColor) -> String {
    // forgot to #[derive(Debug)] in crate, and struggled to implement fmt::Du
    match c {
        SecondaryColor::Green => String::from("Green"),
        SecondaryColor::Orange => String::from("Orange"),
        SecondaryColor::Purple => String::from("Purple"),
    }
}

use day_15::solutions::{get_gps_coords, get_widened_gps_coords};

fn main() {
    let filename = "input.txt".to_string();

    let gps_coords = get_gps_coords(&filename);

    println!("GPS Coords: {gps_coords}");

    let widened_gps_coords = get_widened_gps_coords(&filename);

    println!("GPS coords in the widened map: {widened_gps_coords}");
}

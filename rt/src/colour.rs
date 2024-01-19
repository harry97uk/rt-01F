use std::{ fs::File, io::Write };

use crate::vector3::Vector3;

pub type Colour = Vector3;

pub fn write_colour(file: &mut File, colour: Colour) {
    let ir = (255.999 * colour.x()) as i32;
    let ig = (255.999 * colour.y()) as i32;
    let ib = (255.999 * colour.z()) as i32;

    file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("write failed");
}

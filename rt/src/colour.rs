use std::{ fs::File, io::Write };

use crate::{ vector3::Vector3, interval::Interval };

pub type Colour = Vector3;

pub fn write_colour(file: &mut File, colour: Colour, samples_per_pixel: i32) {
    let mut r = colour.x();
    let mut g = colour.y();
    let mut b = colour.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let intensity: Interval = Interval::new(0.0, 0.999);

    file.write(
        format!(
            "{} {} {}\n",
            256.0 * intensity.clamp(r),
            256.0 * intensity.clamp(g),
            256.0 * intensity.clamp(b)
        ).as_bytes()
    ).expect("write failed");
}

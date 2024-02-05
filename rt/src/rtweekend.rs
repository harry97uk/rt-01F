use rand;

// Constants

//const INFINITY: f64 = std::f64::INFINITY;
const PI: f64 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return (degrees * PI) / 180.0;
}

pub fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    return rand::random::<f64>();
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    return min + (max - min) * random_f64();
}

// pub fn random_int(min: i32, max: i32) -> i32 {
//     // Returns a random integer in [min,max].
//     return random_f64_range(min as f64, (max as f64) + 1.0) as i32;
// }

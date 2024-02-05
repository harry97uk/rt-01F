use std::f64::{ INFINITY, NEG_INFINITY };

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(_min: f64, _max: f64) -> Self {
        Self { min: _min, max: _max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        Interval { min: f64::min(a.min, b.min), max: f64::max(a.max, b.max) }
    }

    pub fn empty() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }

    // pub fn universe() -> Self {
    //     Self { min: NEG_INFINITY, max: INFINITY }
    // }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        return Interval::new(self.min - padding, self.max + padding);
    }
}

// const EMPTY_INTERVAL: Interval = Interval { min: INFINITY, max: NEG_INFINITY };
// const UNIVERSE_INTERVAL: Interval = Interval { min: NEG_INFINITY, max: INFINITY };

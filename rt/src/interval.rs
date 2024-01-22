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

    pub fn empty() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }

    pub fn universe() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

const EMPTY_INTERVAL: Interval = Interval { min: INFINITY, max: NEG_INFINITY };
const UNIVERSE_INTERVAL: Interval = Interval { min: NEG_INFINITY, max: INFINITY };

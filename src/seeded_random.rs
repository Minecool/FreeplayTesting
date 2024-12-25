pub struct SeededRandom {
    pub seed: i64,
}

impl SeededRandom {
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }

    fn next(&mut self) -> i64 {
        self.seed = (self.seed * 16807) % 0x7FFFFFFF;
        self.seed
    }

    pub fn next_float(&mut self) -> f32 {
        (self.next() as f32) / (0x7FFFFFFE as f32)
    }

    pub fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (0x7FFFFFFE as f64)
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        let (lower_bound, upper_bound) = if min > max { (max, min) } else { (min, max) };

        if lower_bound == upper_bound {
            return lower_bound;
        }

        let remainder = self.next();
        let res = lower_bound + (remainder as i32) % (upper_bound - lower_bound);
        res
    }
}

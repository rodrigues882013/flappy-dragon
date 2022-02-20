use bracket_lib::prelude::RandomNumberGenerator;

pub struct Barrier {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Barrier {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Barrier {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }
}
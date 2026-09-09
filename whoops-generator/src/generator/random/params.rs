pub struct Params {
    /// target width of the output grid
    pub width: u32,

    /// target height of the output grid
    pub height: u32,

    /// minimal ratio of walls in the output grid
    /// wall-ratio <= walls / (rows * cols)
    pub wall_low_bound: f32,

    /// maximal ratio of known tiles in the output grid
    /// max-known-ratio >= known / (rows * cols)
    pub known_high_bound: f32,

    /// the amount of tries to perform in order to find the best matching grid to this requirements
    pub iterations: u32,
}

impl Params {
    pub fn new(width: u32, height: u32) -> Self {
        let tiles = width * height;

        let wall_low_bound = 0.2;
        let known_high_bound = 0.35;
        let iterations = 100 / tiles + 1;

        Self {
            width,
            height,
            wall_low_bound,
            known_high_bound,
            iterations,
        }
    }
}

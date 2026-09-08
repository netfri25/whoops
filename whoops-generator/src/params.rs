pub struct GeneratorParams {
    /// target width of the output grid
    pub width: usize,

    /// target height of the output grid
    pub height: usize,

    /// minimal ratio of wall tiles in the output grid
    /// walls_ratio <= walls / (width * height)
    pub walls_low_bound: f64,

    /// maximal ratio of known tiles in the output grid
    /// knowns_ratio >= known / (width * height)
    pub knowns_high_bound: f64,

    /// the amount of tries to perform in order to find the best matching grid to this requirements
    pub iterations: usize,
}

impl GeneratorParams {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = width * height;

        // with some trail and error, I found these values to work well.
        let walls_low_bound = 0.2;
        let knowns_high_bound = 0.35;
        let iterations = 100 / tiles + 1;

        Self {
            width,
            height,
            walls_low_bound,
            knowns_high_bound,
            iterations,
        }
    }
}

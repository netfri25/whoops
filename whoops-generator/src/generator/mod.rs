pub mod fixed;
pub mod random;

/// ability to generate a grid and its solution
pub trait Generator<G> {
    fn generate(self) -> GeneratorOutput<G>;
}

#[derive(Clone)]
pub struct GeneratorOutput<G> {
    pub grid: G,
    pub solution: G,
}

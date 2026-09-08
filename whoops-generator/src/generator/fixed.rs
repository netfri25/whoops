use crate::generator::{Generator, GeneratorOutput};

#[derive(Clone)]
pub struct FixedGenerator<G>(GeneratorOutput<G>);

impl<G> FixedGenerator<G> {
    pub fn new(output: GeneratorOutput<G>) -> Self {
        Self(output)
    }
}

impl<G> From<GeneratorOutput<G>> for FixedGenerator<G> {
    fn from(value: GeneratorOutput<G>) -> Self {
        Self::new(value)
    }
}

impl<G> Generator<G> for FixedGenerator<G> {
    fn generate(self) -> Option<GeneratorOutput<G>> {
        Some(self.0)
    }
}

impl<G> Generator<G> for &FixedGenerator<G>
where
    G: Clone,
{
    fn generate(self) -> Option<GeneratorOutput<G>> {
        Some(self.0.clone())
    }
}

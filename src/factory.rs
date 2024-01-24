use crate::registry::SeedRegistry;

struct Factory {
    iterations: u32,
    registry: SeedRegistry
}

impl Factory {

    fn new() -> Factory {
        Factory {
            iterations: 1,
            registry: SeedRegistry::new()
        }
    }

    fn set_iterations(&mut self, iterations: u32) -> &mut Factory {
        self.iterations = iterations;
        self
    }

    /// Run the factory to generate the seeds
    fn run_factory(&self) {

    }
}
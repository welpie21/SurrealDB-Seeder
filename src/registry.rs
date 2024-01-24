use std::any::Any;
use std::collections::HashMap;
use crate::seed::Seed;

pub struct SeedRegistry {

    /// a dictionary of all the seeds
    seeds: HashMap<&'static str, Seed>,

    ///
    tables: Vec<&'static str>
}

impl SeedRegistry {

    pub fn new() -> SeedRegistry {

        SeedRegistry {
            seeds: HashMap::new(),
            tables: Vec::new()
        }
    }

    pub fn register(&mut self, id: &'static str, seed: Seed) {
        self.seeds.insert(id, seed);
    }

    pub fn get(&self, id: &'static str) -> Option<&Seed> {
        self.seeds.get(id)
    }

    pub fn run_seed(&self, id: &'static str) -> Result<&dyn Any, &'static str> {
        match self.seeds.get(id) {
            Some(seed) => {
                match seed.run {
                    Some(ref run) => {
                        match run() {
                            Ok(result) => Ok(result),
                            Err(_) => Err("Error running seed")
                        }
                    },
                    None => Err("Seed has no run function")
                }
            },
            None => Err("Seed not found")
        }
    }
}
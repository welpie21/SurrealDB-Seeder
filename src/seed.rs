use std::fmt::Error;
use std::any::Any;

pub struct Seed {
    id: &'static str,
    pub(crate) run: Option<dyn Fn() -> Result<dyn Any, Error>>
}

impl Seed {

    fn new(id: &'static str) -> Seed {
        Seed {
            id,
            run: None
        }
    }

    pub fn set_run(&mut self, run: impl Fn() -> Result<dyn Any, Error> + 'static) -> &mut Seed {
        self.run = Some(Box::new(run));
        self
    }
}
pub struct Factory<T: Default> {
	table: &str,
	iterations: u32
}

impl<T> Factory<T> {

	pub fn table(table: &str) -> Self {
		Factory {
			table,
			factory: T::default(),
			iterations: 1
		}
	}

	pub fn iterations(mut self, iterations: u32) -> Self {
		self.iterations = iterations;
		self
	}
}

#[derive(Default)]
struct Beaulder {
	name: String,
	age: u8,
	bitches: Vec<String>
}

fn make_beaulder() -> Beaulder {
	let beau = Beaulder {
		..Default::default(),
	};
}
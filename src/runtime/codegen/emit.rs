use runtime::jit::function::Function;
pub struct Writer {
	stream: Vec<u8>
}

impl Writer {
	pub fn new() -> Writer {
		Writer { stream: Vec::new() }
	}

	pub fn write(&mut self, b: u8) {
		self.stream.push(b);
	}

	pub fn write_all(&mut self, bs: &[u8]) {
		for b in bs {
            self.stream.push(*b);
        }
	}

	pub fn len(&self) -> usize { 
		self.stream.len() 
	}

	pub fn build(&self) -> Function {
	    Function::new(&self.stream)
	}
}
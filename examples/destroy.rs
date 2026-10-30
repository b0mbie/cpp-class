use cpp_class::class;

class! {
	pub IResources {
		pub virtual fn get(i: usize) -> *mut IResource;
	}
}

class! {
	pub IDisposable {
		pub virtual ~destroy();
	}
}

class! {
	pub IResource: pub virtual IDisposable {
		// It is a compile-time error to declare another destructor:
		// pub virtual ~destroy();
	}
}

fn main() {
}

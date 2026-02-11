use cpp_class::class;

class! {
	pub Index<T> {
		pub virtual fn get(index: usize) -> *const T;
	}
}

class! {
	pub IndexMut<T> {
		pub virtual fn get_mut(index: usize) -> *mut T;
	}
}

class! {
	pub Length {
		pub virtual fn length() -> usize;
	}
}

class! {
	pub List<T>: pub virtual Length, pub virtual Index<T>, pub virtual IndexMut<T> {
		pub virtual use fn(T) as _t;
	}
}

class! {
	pub Length2<T> {
		pub virtual fn length() -> usize;
		pub elements: *mut T;
		pub length: usize;
	}
}

fn main() {}

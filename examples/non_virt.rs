use cpp_class::class;

class! {
	pub Vec3<T> {
		pub x: T;
		pub y: T;
		pub z: T;
	}
}

class! {
	pub Vec4<T>: pub Vec3<T> {
		pub w: T;
	}
}

class! {
	pub Vec3f: pub Vec3<f32> {}
}

class! {
	pub Vec4f: pub Vec4<f32> {}
}

fn main() {}

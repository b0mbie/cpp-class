use crate::{
	Virtual, Destructor,
};

#[diagnostic::on_unimplemented(
	message = "cannot shadow virtual destructor of `{Self}`",
	label = "`{Self}` declares virtual destructor",
	note = "virtual destructors cannot be declared if the inherited class already does so",
	note = "consider removing the virtual destructor declaration in the deriving class",
)]
pub unsafe trait HasNoDestructor: Virtual {}

// TODO: There are typically two destructors - one for dynamically-allocated data, and one for everything else.
// However, MSVC generates only one destructor function with some unspecified hidden parameter.
// Maybe it's possible to somehow model that? What is that parameter's type?
#[repr(C)]
pub struct Destructors<T> {
	destructor_1: Destructor<T>,
	#[cfg(not(windows))]
	destructor_2: Destructor<T>,
}

impl<T> Destructors<T> {
	pub const fn get(self) -> Destructor<T> {
		self.destructor_1
	}

	pub const fn new(destructor: Destructor<T>) -> Self {
		Self {
			destructor_1: destructor,
			#[cfg(not(windows))]
			destructor_2: destructor,
		}
	}
}

impl<T> Clone for Destructors<T> {
	fn clone(&self) -> Self {*self}
}
impl<T> Copy for Destructors<T> {}

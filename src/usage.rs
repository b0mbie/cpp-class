use core::{
	any::type_name,
	fmt,
	marker::PhantomData,
};

#[repr(transparent)]
pub struct Usage<T: ?Sized>(pub PhantomData<T>);

impl<T: ?Sized> Usage<T> {
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: ?Sized> Clone for Usage<T> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T: ?Sized> Copy for Usage<T> {}

impl<T: ?Sized> Default for Usage<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: ?Sized> fmt::Debug for Usage<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Usage<")?;
		f.write_str(type_name::<T>())?;
		f.write_str(">")
	}
}

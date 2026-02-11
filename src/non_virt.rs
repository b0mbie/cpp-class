/// Trait for types representing a non-virtual C++ class.
/// 
/// # Safety
/// The safety requirements of this trait will always be upheld by
/// types created with the [`class!`] macro.
/// 
/// The implementing type must not also implement [`Virtual`](super::Virtual).
#[diagnostic::on_unimplemented(message = "`{Self}` is not a C++ class, or is a virtual class")]
pub unsafe trait NonVirtual {
	/// This should always be set to `Self`.
	type This;
}

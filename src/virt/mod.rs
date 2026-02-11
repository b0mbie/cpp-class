mod vtable_ptr;
pub use vtable_ptr::*;

/// Trait for types representing a virtual C++ class.
/// 
/// The safety requirements of this trait will always be upheld by
/// types created with the [`class!`] macro.
/// 
/// # Safety
/// The implementing type must not also implement [`NonVirtual`](super::NonVirtual).
/// 
/// The memory layout of the type must
/// include [`VTablePtr<Self::VTable>`](VTablePtr) as the first `repr(C)` field,
/// and [`Self::Data`](Virtual::Data) as the field after that.
#[diagnostic::on_unimplemented(message = "`{Self}` is not a C++ virtual class")]
pub unsafe trait Virtual {
	/// This should always be set to `Self`.
	type This;

	/// Type of the virtual function table used for the class.
	type VTable;
	/// Type for the data portion stored in the class.
	type Data;
}

/// Returns the [`VTablePtr`] of the given [`Virtual`] class instance.
pub const fn vtable_of<T: Virtual>(t: &T) -> VTablePtr<T::VTable> {
	// SAFETY: `VTablePtr<T::VTable>` is the first `repr(C)` field (at offset `0`).
	unsafe { (t as *const T as *const VTablePtr<T::VTable>).read() }
}

/// Type for `this` pointers for [`Virtual`] class functions.
pub type This<T> = core::ptr::NonNull<T>;

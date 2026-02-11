use core::{
	cmp::Ordering,
	fmt,
	hash::{
		Hash, Hasher,
	},
	ptr::NonNull,
};

/// Type for virtual function table pointers.
/// 
/// Pointers of this type either point to a
/// shared, immutable `VTable`,
/// or an exclusive, mutable `VTable`.
/// Due to this, unlike [`NonNull<VTable>`],
/// this type implements
/// [`Send`] (the type has a unique pointer) and
/// [`Sync`] (the type does not provide interior mutability)
/// if `VTable` itself is both `Send` and `Sync`.
/// 
/// # Layout
/// This type has the same layout and ABI as [`NonNull<VTable>`].
#[repr(transparent)]
pub struct VTablePtr<VTable>(NonNull<VTable>);

// SAFETY: `VTablePtr<VTable>` exclusively points to a `VTable`.
unsafe impl<VTable: Send> Send for VTablePtr<VTable> {}
// SAFETY: `VTablePtr<VTable>` does not provide interior mutability.
unsafe impl<VTable: Sync> Sync for VTablePtr<VTable> {}

impl<VTable> VTablePtr<VTable> {
	/// Returns a new non-null [`VTablePtr`].
	/// 
	/// # Safety
	/// `ptr` must point to a valid `VTable`.
	pub const unsafe fn new(ptr: NonNull<VTable>) -> Self {
		Self(ptr)
	}

	/// Converts a static immutable reference to a [`VTablePtr`].
	pub const fn from_ref(vtable: &'static VTable) -> Self {
		unsafe { Self::new(NonNull::new_unchecked(vtable as *const _ as *mut _)) }
	}

	/// Converts a static mutable reference to a [`VTablePtr`].
	pub const fn from_mut(vtable: &'static mut VTable) -> Self {
		unsafe { Self::new(NonNull::new_unchecked(vtable as *mut _)) }
	}

	/// Consumes this pointer, converting it into the inner [`NonNull`].
	pub const fn into_ptr(self) -> NonNull<VTable> {
		self.0
	}

	/// Returns an immutable reference to the `VTable`.
	pub const fn as_ref(&self) -> &VTable {
		unsafe { self.0.as_ref() }
	}

	/// Returns a mutable reference to the `VTable`.
	/// 
	/// # Safety
	/// The `VTable` is usually not intended to be modified,
	/// and all sorts of Undefined Behavior may arise from its modification.
	pub const unsafe fn as_mut(&mut self) -> &mut VTable {
		unsafe { self.0.as_mut() }
	}
}

impl<VTable> fmt::Debug for VTablePtr<VTable> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl<VTable> PartialEq for VTablePtr<VTable> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
impl<VTable> Eq for VTablePtr<VTable> {}

impl<VTable> PartialOrd for VTablePtr<VTable> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl<VTable> Ord for VTablePtr<VTable> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

impl<VTable> Hash for VTablePtr<VTable> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state)
	}
}

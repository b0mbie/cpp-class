mod cc;
pub use cc::*;

mod virtual_call;
mod virtual_fn;

/// Given `$type` is a VTable type and `Self` has all of the virtual methods for
/// that VTable with the same name, create a new VTable with those methods.
/// 
/// Optionally, you can specify `for $self:ident`, where `$self` is the type to
/// use instead of `Self`.
#[macro_export]
macro_rules! new_vtable_self {
	{
		@init
		()
		($($t:tt)*)
	} => {$($t)*};
	{
		@init
		($($i:tt)+)
		($($t:tt)*)
	} => {$($i)*};

	(
		$type:ident {
			$(
				$(#[$set_attr:meta])*
				$func_name:ident $(: $or_init:expr)?
			),*
			$(,)?
		}
	) => {
		$type {
			$(
                $(#[$set_attr])*
				$func_name: $crate::new_vtable_self! {
					@init ($($or_init)?) (Self::$func_name)
				},
			)*
		}
	};

	(
		$type:ident for $self:ident {
			$(
				$(#[$set_attr:meta])*
				$func_name:ident
			),*
		}
	) => {
		$type {
			$(
                $(#[$set_attr])*
				$func_name: $crate::new_vtable_self! {
					@init ($($or_init)?) (Self::$func_name)
				},
			)*
		}
	};
}

/// Convert the pointer `$this` to a probably-`mut` reference to `Self`.
#[macro_export]
macro_rules! this_to_self {
	(mut $this:expr) => {{
		let this: $crate::This<_> = $this;
		this.cast::<Self>().as_mut()
	}};

	(ref $this:expr) => {{
		let this: $crate::This<_> = $this;
		this.cast::<Self>().as_ref()
	}};
}

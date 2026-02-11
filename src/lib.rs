#![no_std]

#[doc(hidden)]
pub use cpp_class_macros::class_impl;

#[macro_export]
macro_rules! class {
	($($tt:tt)*) => {
		$crate::class_impl! {
			$crate
			$($tt)*
		}
	};
}

mod macros;

mod inheritance;
pub use inheritance::*;
mod non_virt;
pub use non_virt::*;
mod usage;
pub use usage::*;
mod virt;
pub use virt::*;

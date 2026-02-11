#![allow(unused_macros)]

/// Given an input of `pre = {/* ... */}; post = {/* ... */};`,
/// expands to a series of tokens that wraps the calling convention for
/// non-variadic functions
/// in tokens of `pre` and `post`.
#[macro_export]
macro_rules! non_variadic {
	{$($tt:tt)*} => {
		$crate::non_variadic_impl! {$($tt)*}
	};
}

/// Given an input of `pre = {/* ... */}; post = {/* ... */};`,
/// expands to a series of tokens that wraps the calling convention for
/// variadic functions
/// in tokens of `pre` and `post`.
#[macro_export]
macro_rules! variadic {
	{$($tt:tt)*} => {
		$crate::variadic_impl! {$($tt)*}
	};
}

/// Given an input of `pre = {/* ... */}; post = {/* ... */};`,
/// expands to a series of tokens that wraps the calling convention for
/// non-variadic unwinding functions
/// in tokens of `pre` and `post`.
#[macro_export]
macro_rules! unwind {
	{$($tt:tt)*} => {
		$crate::unwind_impl! {$($tt)*}
	};
}

/// Given an input of `pre = {/* ... */}; post = {/* ... */};`,
/// expands to a series of tokens that wraps the calling convention for
/// variadic unwinding functions
/// in tokens of `pre` and `post`.
#[macro_export]
macro_rules! variadic_unwind {
	{$($tt:tt)*} => {
		$crate::variadic_unwind_impl! {$($tt)*}
	};
}

macro_rules! cc_macros {
	{
		// We need the `$` symbol for generating `macro_rules!`.
		@($d:tt)
		$(
			$(#[$macro_attr:meta])*
			macro $macro:ident {
				cc = $cc:literal;
			}
		)*
	} => {
		$(
			$(#[$macro_attr])*
			macro_rules! $macro {
				{
					pre = {$d($pre:tt)*};
					post = {$d($post:tt)*};
				} => {
					$d($pre)* $cc $d($post)*
				};
			}
		)*
	};
	{@($d:tt) $($whatever:tt)*} => {
		::core::compile_error! {"invalid macro invocation"}
	};
	($($arg:tt)*) => {
		cc_macros!(@($) $($arg)*);
	};
}

cc_macros! {
	#[cfg(not(all(windows, target_arch = "x86")))]
	#[doc(hidden)]
	#[macro_export]
	macro non_variadic_impl {
		cc = "C";
	}

	#[cfg(all(windows, target_arch = "x86"))]
	#[doc(hidden)]
	#[macro_export]
	macro non_variadic_impl {
		cc = "thiscall";
	}

	#[cfg(not(all(windows, target_arch = "x86")))]
	#[doc(hidden)]
	#[macro_export]
	macro unwind_impl {
		cc = "C-unwind";
	}

	#[cfg(all(windows, target_arch = "x86"))]
	#[doc(hidden)]
	#[macro_export]
	macro unwind_impl {
		cc = "thiscall-unwind";
	}

	#[doc(hidden)]
	#[macro_export]
	macro variadic_impl {
		cc = "C";
	}

	#[doc(hidden)]
	#[macro_export]
	macro variadic_unwind_impl {
		cc = "C-unwind";
	}
}

/// Expands to a `fn` type or `fn` item
/// that represents a C++ virtual method
/// with the correct calling convention selected for the target.
/// 
/// See also [`unwind_virtual_fn!`](crate::unwind_virtual_fn!).
/// 
/// # Examples
/// ```
/// # use cppdvt::{VtObjectPtr, virtual_fn};
/// # #[repr(C)] struct RngVt {}
/// // `RngVt` is some C++ VTable.
/// type NextI32Fn = virtual_fn!(fn(this: VtObjectPtr<RngVt>) -> i32);
/// virtual_fn! {
///     fn next_i32(this: VtObjectPtr<RngVt>) -> i32 {
///         // Chosen by fair dice roll. Guaranteed to be random.
///         193691999
///     }
/// }
/// 
/// // Types will always match.
/// let _: NextI32Fn = next_i32;
/// // Types may or may not match on different targets!
/// // let _: unsafe extern "thiscall" fn(VtObjectPtr<RngVt>) = next_i32;
/// ```
#[macro_export]
macro_rules! virtual_fn {
	{
		$(#[$attr:meta])*
		fn $($name:ident)?
		$([$($generic:tt)*])?
		($($param:tt)*)
		$($tt:tt)*
	} => {
		$crate::if_variadic! {
			{
				$crate::variadic! {
					pre = {$(#[$attr])* unsafe extern};
					post = {
						fn $($name)?
						$(<$($generic)*>)?
						($($param)*) $($tt)*
					};
				}
			} else {
				$crate::non_variadic! {
					pre = {$(#[$attr])* unsafe extern};
					post = {
						fn $($name)?
						$(<$($generic)*>)?
						($($param)*) $($tt)*
					};
				}
			}
			$($param)*
		}
	};
}

/// Expands to a `fn` type or `fn` item
/// that represents a C++ virtual method
/// with the correct `*-unwind` calling convention selected for the target.
/// 
/// See also [`virtual_fn!`](crate::virtual_fn!).
/// 
/// # Examples
/// ```
/// # use cppdvt::{VtObjectPtr, unwind_virtual_fn};
/// # #[repr(C)] struct AppVt {}
/// // `AppVt` is some C++ VTable.
/// type MainFn = unwind_virtual_fn!(fn(this: VtObjectPtr<AppVt>));
/// unwind_virtual_fn! {
///     fn app_main(this: VtObjectPtr<AppVt>) {}
/// }
/// 
/// // Types will always match.
/// let _: MainFn = app_main;
/// // Types may or may not match on different targets!
/// // let _: unsafe extern "thiscall-unwind" fn(VtObjectPtr<AppVt>) = app_main;
/// ```
#[macro_export]
macro_rules! unwind_virtual_fn {
	{
		$(#[$attr:meta])*
		fn $($name:ident)?
		$([$($generic:tt)*])?
		($($param:tt)*)
		$($tt:tt)*
	} => {
		$crate::if_variadic! {
			{
				$crate::variadic_unwind! {
					pre = {$(#[$attr])* unsafe extern};
					post = {
						fn $($name)?
						$(<$($generic)*>)?
						($($param)*) $($tt)*
					};
				}
			} else {
				$crate::unwind! {
					pre = {$(#[$attr])* unsafe extern};
					post = {
						fn $($name)?
						$(<$($generic)*>)?
						($($param)*) $($tt)*
					};
				}
			}
			$($param)*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! if_variadic {
	{
		{$($then:tt)*} else {$($else:tt)*}
		...
	} => {
		$($then)*
	};

	{
		{$($then:tt)*} else {$($else:tt)*}
	} => {
		$($else)*
	};

	{
		{$($then:tt)*} else {$($else:tt)*}
		$param1:tt $($param:tt)*
	} => {
		$crate::if_variadic! {
			{$($then)*} else {$($else)*}
			$($param)*
		}
	};
}

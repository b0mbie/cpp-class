/// Given an invokation of the form `(mut)? <instance> => path.to.func(...)`,
/// invoke the virtual method `func`
/// of the [`Virtual`](crate::Virtual) type
/// with the specified arguments, if any.
#[macro_export]
macro_rules! virtual_call {
	(mut $instance:expr => $field:ident$(.$suffix:ident)*($($arg:tt)*)) => {{
		let mut instance = $instance;
		let vtable = $crate::vtable_of(instance);
		let this = $crate::This::from_mut(instance);
		(vtable.as_ref().$field$(.$suffix)*)(this, $($arg)*)
	}};

	($instance:expr => $field:ident$(.$suffix:ident)*($($arg:tt)*)) => {{
		let instance = $instance;
		let vtable = $crate::vtable_of(instance);
		let this = $crate::This::from_ref(instance);
		(vtable.as_ref().$field$(.$suffix)*)(this, $($arg)*)
	}};

	($($whatever:tt)*) => {
		::core::compile_error! {
			"expected invocation of the form `(mut)? <instance> => path.to.func(...)`"
		}
	};
}

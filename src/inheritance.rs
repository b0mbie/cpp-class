/// Marker trait indicating that
/// the implementing C++ class inherits from `Base`.
/// 
/// The safety requirements of this trait will always be upheld by
/// types created with the [`class!`] macro.
/// 
/// # Safety
/// The implementing type must be
/// a C++ class that inherits from `Base`.
#[diagnostic::on_unimplemented(message = "`{Self}` is not a C++ class that inherits from `{Base}`")]
pub unsafe trait Inherits<Base> {}

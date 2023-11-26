// Implicit export macros from derive crate.
#[macro_use]
#[allow(unused_imports)]
extern crate field_count_derive;

// Export everything from derive crate
pub use field_count_derive::*;

/// A data structure the exposes the number of fields it has.
///
/// This trait can be derived:
///
/// ```
/// use field_count::FieldCount;
///
/// #[derive(FieldCount)]
/// struct MyStruct
/// {
///    first_field: i32,
///    second_field: String,
///    third_field: u16,
/// }
///
/// println!("{}", MyStruct::field_count()); // 3
/// ```
pub trait FieldCount {
    /// Get the number of fields on a struct.
    fn field_count() -> usize;
}

/// An enum that exposes the number of fields each of its variants have based on a runtime value.
///
/// This trait can be derived:
///
/// ```
/// use field_count::EnumFieldCount;
///
/// #[derive(EnumFieldCount)]
/// enum MyEnum
/// {
///     A,
///     B(usize),
///     C(Vec<u8>, bool),
/// }
///
/// println!("{}", MyEnum::C(vec![], true).field_count()); // 2
/// ```
pub trait EnumFieldCount {
    /// Get the number of fields this enum variant has.
    fn field_count(&self) -> usize;
}

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

// Export derive macro from derive crate.
pub use field_count_derive::*;

#[cfg(test)]
mod tests {
    use super::{FieldCount, EnumFieldCount};

    #[test]
    fn test_derive_field_count_for_struct() {
        assert_eq!(MyStruct::field_count(), 3);
    }

    #[test]
    fn test_derive_field_count_for_generic_struct() {
        assert_eq!(MyGenericStruct::<u32>::field_count(), 1);
    }

    #[test]
    fn test_derive_field_count_as_trait() {
        assert_eq!(field_count::<MyStruct>(), 3);
    }

    fn field_count<T: FieldCount>() -> usize {
        T::field_count()
    }

    #[test]
    fn test_derive_variant_field_count_for_enum() {
        let a = MyEnum::A;
        let b = MyEnum::B(32);
        let c = MyEnum::C(vec![], true);

        assert_eq!(a.field_count(), 0);
        assert_eq!(b.field_count(), 1);
        assert_eq!(c.field_count(), 2);
    }

    #[test]
    fn test_derive_variant_field_count_for_generic_enum() {
        let b: MyGenericEnum<i32> = MyGenericEnum::Basic;
        let g = MyGenericEnum::Gen::<Vec<i32>>(vec![1, 2, 3]);

        assert_eq!(b.field_count(), 0);
        assert_eq!(g.field_count(), 1);
    }

    #[test]
    fn test_derive_variant_field_count_as_trait() {
        let c = MyEnum::C(vec![], true);
        assert_eq!(field_count_enum(&c), 2);
    }

    fn field_count_enum<T: EnumFieldCount>(v: &T) -> usize {
        v.field_count()
    }

    #[derive(FieldCount)]
    struct MyStruct {
        _first_field: i32,
        _second_field: String,
        _third_field: u16,
    }

    #[derive(FieldCount)]
    struct MyGenericStruct<T> {
        _generic_field: T,
    }

    #[derive(EnumFieldCount)]
    enum MyEnum {
        A,
        B(usize),
        C(Vec<u8>, bool),
    }

    #[derive(EnumFieldCount)]
    enum MyGenericEnum<T> {
        Basic,
        Gen(T),
    }
}

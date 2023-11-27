#![allow(unused)]

#[macro_use]
#[allow(unused_imports)]
extern crate field_count;

use ::field_count::{EnumFieldCount, FieldCount};

#[test]
fn test_derive_field_count_for_struct() {
    assert_eq!(MyStruct::field_count(), 3);
}

#[test]
fn test_derive_const_field_count_for_struct() {
    const COUNT: usize = MyStruct::field_count();
    assert_eq!(COUNT, 3);
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
pub struct MyStruct {
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

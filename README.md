# field_count

Derive the field count for a struct / enum variants, generating implementations for the `FieldCount`
and `EnumFieldCount` traits, respectively.

This is fork and continuation of the original [field_count](https://github.com/discosultan/field-count) project that is
unmaintained.

## Getting Started

```toml
# Cargo.toml

[dependencies]
field_count = { git = "https://github.com/rushiiMachine/field-count" }
```

```rust
use field_count::FieldCount;

// Two impls are generated, one implementing the FieldCount trait,
// and the other a freestanding impl that has a const version of `field_count`
#[derive(FieldCount)]
struct MyStruct {
    first_field: i32,
    second_field: String,
    third_field: u16,
}

fn main() {
    assert_eq!(MyStruct::field_count(), 3);
}
```

```rust
use field_count::EnumFieldCount;

#[derive(EnumFieldCount)]
enum MyGenericEnum<T> {
    Basic,
    Gen(T),
}

fn main() {
    let b: MyGenericEnum = MyGenericEnum::Basic::<i32>;
    let g = MyGenericEnum::Gen(vec![1, 2, 3]);

    assert_eq!(b.field_count(), 0);
    assert_eq!(g.field_count(), 1);
}
```

## Credits

This crate was inspired by [the following StackOverflow answer](https://stackoverflow.com/a/54177920/1466456)
by [Lukas Kalbertodt](https://github.com/LukasKalbertodt).

### Fork changes

- Add support for enums via `#[derive(VariantFieldCount)]` and `VariantFieldCount` trait

<details>
  <summary>Usage</summary>

```rust
// main.rs

use field_count::VariantFieldCount;

#[derive(VariantFieldCount)]
enum MyEnum {
    A,
    B(usize),
    C(Vec<u8>, bool),
}

fn main() {
    let a = MyEnum::A;
    let b = MyEnum::B(128);
    let c = MyEnum::C(vec![], true);
    println!("{}", a.field_count()); // 0
    println!("{}", b.field_count()); // 1
    println!("{}", c.field_count()); // 2
}
```

</details>

<sub>Original README below</sub>

# field_count

Derive the field count for a struct. Implements a `FieldCount` trait. Supports generic structs.

## 📦 Getting Started

```toml
# Cargo.toml

[dependencies]
field_count = "0.1"
```

```rust
// main.rs

use field_count::FieldCount;

#[derive(FieldCount)]
struct MyStruct {
    first_field: i32,
    second_field: String,
    third_field: u16,
}

fn main() {
    println!("{}", MyStruct::field_count()); // 3
}
```

## 🙏 Credits

This crate was inspired by [the following StackOverflow answer](https://stackoverflow.com/a/54177920/1466456) by [Lukas Kalbertodt](https://github.com/LukasKalbertodt).

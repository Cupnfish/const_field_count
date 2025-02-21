# Const Field Count

Const Field Count is a Rust procedural macro that allows you to derive a constant function to count the number of fields in a struct. This can be useful for compile-time checks and optimizations.

## Usage

To use the `FieldCount` derive macro, add the following to your `Cargo.toml`:

```toml
[dependencies]
const_field_count = "0.1.0"
```

Then, in your Rust code, you can derive the `FieldCount` trait for your structs:

```rust
use const_field_count::FieldCount;

#[derive(FieldCount)]
struct TokenInfo {
    _name: String,
    _price: i32,
}

fn main() {
    assert_eq!(TokenInfo::field_count(), 2);
}
```

## Limitations

- The `FieldCount` macro can only be derived for structs. Attempting to use it on enums or unions will result in a compile-time panic.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

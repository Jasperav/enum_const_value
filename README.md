# Enum const value

[![Latest Version](https://img.shields.io/crates/v/enum_const_value.svg)](https://crates.io/crates/enum_const_value)
[![Build Status](https://img.shields.io/github/workflow/status/jasperav/enum_const_value/CI/master)](https://github.com/jasperav/enum_const_value/actions)

A derive macro that will add const values for enum cases, even enums with associated types.
When dealing with an enum with associated types, the macro creates a new enum with the const values.

## Usage

```toml
[dependencies]
enum_const_value = "0.1.0"
```

Now in your project, add the following:

```rust
use enum_const_value::EnumConstValue;
```

and use the derive macro on an enum
```rust
#[derive(EnumConstValue)]
enum MyEnum {
    SomeI32(i32),
    SomeString(String)
}
```

Now, you can use use the `const_value()` method on an enum case like so:
```rust
#[test]
fn test_my_enum() {
    assert_eq!(0, MyEnum::SomeI32(1).const_value());
    assert_eq!(1, MyEnum::SomeString("MyString".to_string()).const_value());

    assert_eq!(0, MyEnumConstValue::SomeI32.value());
    assert_eq!(1, MyEnumConstValue::SomeString.value());
}
```

## TODO
- Custom enum case values (can be done by adding attributes)
- Custom derived enum name (is currently always $ENUMNAME$ConstValue)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

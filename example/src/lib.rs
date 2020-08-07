use enum_const_value::EnumConstValue;

#[derive(EnumConstValue)]
enum MyEnum {
    SomeI32(i32),
    SomeString(String)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_enum() {
        assert_eq!(0, MyEnum::SomeI32(1).const_value());
        assert_eq!(1, MyEnum::SomeString("MyString".to_string()).const_value());

        assert_eq!(0, MyEnumConstValue::SomeI32.value());
        assert_eq!(1, MyEnumConstValue::SomeString.value());
    }
}
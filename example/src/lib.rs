use enum_const_value::EnumConstValue;

#[derive(EnumConstValue)]
enum ValueEnum {
    SomeCase(i32),
    AnotherCase(String)
}

#[derive(EnumConstValue)]
enum PlainEnum {
    SomeCase,
    AnotherCase
}

#[derive(EnumConstValue)]
enum MixedEnum {
    SomeCase,
    AnotherCase(String)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_enum() {
        assert_eq!(0, ValueEnum::SomeCase(1).const_value());
        assert_eq!(1, ValueEnum::AnotherCase("MyString".to_string()).const_value());

        assert_eq!(0, ValueEnumConstValue::SomeCase.value());
        assert_eq!(1, ValueEnumConstValue::AnotherCase.value());

        assert_eq!(0, PlainEnum::SomeCase.const_value());
        assert_eq!(1, PlainEnum::AnotherCase.const_value());

        assert_eq!(0, MixedEnum::SomeCase.const_value());
        assert_eq!(1, MixedEnum::AnotherCase("MyString".to_string()).const_value());

        assert_eq!(0, MixedEnumConstValue::SomeCase.value());
        assert_eq!(1, MixedEnumConstValue::AnotherCase.value());
    }
}
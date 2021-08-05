use enum_const_value::EnumConstValue;

#[derive(EnumConstValue)]
enum PlainEnum {
    SomeCase,
    AnotherCase
}

#[derive(EnumConstValue)]
enum ValueEnum {
    SomeCase(i32),
    AnotherCase(String)
}

#[derive(EnumConstValue)]
enum CLikeStructEnum {
    #[allow(dead_code)]
    SomeCase { x: i32 },
    #[allow(dead_code)]
    AnotherCase { y: String }
}

#[derive(EnumConstValue)]
enum MixedEnum {
    SomeCase,
    AnotherCase(String),
    #[allow(dead_code)]
    CLikeStruct { x: String },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_enum() {
        assert_eq!(0, PlainEnum::SomeCase.const_value());
        assert_eq!(1, PlainEnum::AnotherCase.const_value());
        assert_eq!("AnotherCase", PlainEnum::AnotherCase.retrieve_ident());

        assert_eq!(0, ValueEnum::SomeCase(1).const_value());
        assert_eq!(1, ValueEnum::AnotherCase("MyString".to_string()).const_value());
        assert_eq!(0, ValueEnumConstValue::SomeCase.const_value());
        assert_eq!(1, ValueEnumConstValue::AnotherCase.const_value());
        assert_eq!("AnotherCase", ValueEnum::AnotherCase("MyString".to_string()).retrieve_ident());

        assert_eq!(0, CLikeStructEnum::SomeCase { x: 1 }.const_value());
        assert_eq!(1, CLikeStructEnum::AnotherCase { y: "MyString".to_string() }.const_value());
        assert_eq!(0, CLikeStructEnumConstValue::SomeCase.const_value());
        assert_eq!(1, CLikeStructEnumConstValue::AnotherCase.const_value());

        assert_eq!(0, MixedEnum::SomeCase.const_value());
        assert_eq!(1, MixedEnum::AnotherCase("MyString".to_string()).const_value());
        assert_eq!(2, MixedEnum::CLikeStruct { x: "MyString".to_string() }.const_value());
        assert_eq!("CLikeStruct", MixedEnum::CLikeStruct { x: "MyString".to_string() }.retrieve_ident());
        assert_eq!(0, MixedEnumConstValue::SomeCase.const_value());
        assert_eq!(1, MixedEnumConstValue::AnotherCase.const_value());
        assert_eq!(2, MixedEnumConstValue::CLikeStruct.const_value());
    }
}
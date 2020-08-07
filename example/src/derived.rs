enum MyEnum {
    SomeI32(i32),
    SomeString(String),
}
impl MyEnum {
    pub fn const_value(&self) -> i32 {
        match self {
            &MyEnum::SomeI32(_) => 0i32,
            &MyEnum::SomeString(_) => 1i32,
        }
    }
}
pub enum MyEnumConstValue {
    SomeI32,
    SomeString,
}
impl MyEnumConstValue {
    pub fn value_for_variant(variant: &MyEnumConstValue) -> i32 {
        match variant {
            &MyEnumConstValue::SomeI32 => 0i32,
            &MyEnumConstValue::SomeString => 1i32,
        }
    }
    pub fn value(&self) -> i32 {
        MyEnumConstValue::value_for_variant(self)
    }
}
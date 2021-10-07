pub use primitive_enum_derive::{PrimitiveFromEnum, FromU8};

/**
Need for complex Enums, which includes other data:
```
enum Complex {
    A(String),
    B(u32),
    C
}
enum Primitive {
    A,
    B,
    C,
}
```
PrimitiveEnum should be equivalent for Complex, but without variants inner data
 */
pub trait PrimitiveFromEnum {
    type PrimitiveEnum;

    fn get_primitive_enum(&self) -> Self::PrimitiveEnum;

    fn primitive_name() -> &'static str;
}

use primitive_enum::PrimitiveFromEnum;
use primitive_enum_derive::FromU8;

#[derive(FromU8, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Primitive {
    A,
    B,
    C,
}

#[derive(PrimitiveFromEnum)]
#[primitive(Primitive)]
enum Complex {
    A(String),
    B(u32),
    C,
}

#[test]
fn simple() {
    let a = Complex::A(String::from("test"));
    let b = Complex::B(123);
    let c = Complex::C;

    assert_eq!(a.get_primitive_enum(), Primitive::A);
    assert_eq!(b.get_primitive_enum(), Primitive::B);
    assert_eq!(c.get_primitive_enum(), Primitive::C);
}

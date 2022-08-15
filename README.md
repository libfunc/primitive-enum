# Macros for get primitive enum from complex

examples:

```rust
#[derive(PrimitiveFromEnum)]
#[coming(primitive = "Primitive")]
enum Complex {
    A(String),
    B(u32),
    C
}
#[derive(FromU8)]
enum Primitive {
    A,
    B,
    C,
}
```

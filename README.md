
# `serde-nu`

Convert any value implementing `serde::Serialize` into a
`nu_protocol::Value` using `serde_nu::to_value`. Compare the below manual
implemeentation and the one using `serde_nu`.

```rust
use nu_protocol::{Dictionary, UntaggedValue, Value, Primitive};
use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {
    index: usize,
    name: String,
}

fn manual(s: MyStruct) -> Value {
    let mut dict = Dictionary::default();
    dict.insert(
        "index".into(),
        Value::from(UntaggedValue::Primitive(Primitive::Int(s.index as i64))),
    );
    dict.insert(
        "name".into(),
        Value::from(UntaggedValue::Primitive(Primitive::String(s.name))),
    );

    Value::from(UntaggedValue::Row(dict))
}

fn auto(s: &MyStruct) -> Value {
    serde_nu::to_value(s).unwrap()
}
```

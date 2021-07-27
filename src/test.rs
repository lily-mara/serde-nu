use super::to_value;
use insta::assert_debug_snapshot;
use serde::Serialize;
use std::collections::BTreeMap;

#[test]
fn it_works_with_single_integers() {
    assert_debug_snapshot!(to_value(&4i32));
}

#[test]
fn it_works_with_lists_of_values() {
    assert_debug_snapshot!(to_value(&vec![4i32, 10, 8843234, 100]));
}

#[test]
fn it_works_with_complex_structs() {
    #[derive(Serialize, Debug)]
    struct Complex {
        index: i64,
        x: f32,
        y: f64,
        map: BTreeMap<String, Vec<u8>>,
    }

    let mut map = BTreeMap::new();

    map.insert("coconuts".into(), vec![4]);
    map.insert("tilapia".into(), vec![16, 3, 24]);
    map.insert("mahi mahi".into(), vec![]);

    assert_debug_snapshot!(to_value(&Complex {
        index: -40,
        x: 32.8,
        y: 38.2,
        map
    }));
}

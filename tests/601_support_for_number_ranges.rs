use ron;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Debug)]
struct RangeTest {
    a: std::ops::Range<i32>,
    b: std::ops::RangeInclusive<i32>,
    c: std::ops::Range<f32>,
    d: std::ops::RangeInclusive<f32>,
}

#[test]
fn test_ranges() {
    let ranges = RangeTest {
        a: 0..5,
        b: 1..=3,
        c: 0.6..4.3,
        d: 0.3..=5.7,
    };

    let ser = ron::to_string(&ranges).unwrap();
    assert_eq!(ser, "(a:0..5,b:1..=3,c:0.6..4.3,d:0.3..=5.7)");

    let de: RangeTest = ron::from_str(&ser).unwrap();
    assert_eq!(de, ranges);
}

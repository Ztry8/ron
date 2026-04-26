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

#[test]
fn test_range_integer_bases() {
    assert_eq!(
        ron::from_str::<std::ops::Range<u8>>("0b0000..0b0101").unwrap(),
        0..5
    );
    assert_eq!(
        ron::from_str::<std::ops::Range<u8>>("0o0..0o5").unwrap(),
        0..5
    );
    assert_eq!(
        ron::from_str::<std::ops::Range<u8>>("0x0..0x5").unwrap(),
        0..5
    );

    assert_eq!(
        ron::from_str::<std::ops::Range<u8>>("b'\\x00'..b'\\x05'").unwrap(),
        0..5
    );
    assert_eq!(
        ron::from_str::<std::ops::RangeInclusive<u8>>("b'A'..=b'Z'").unwrap(),
        b'A'..=b'Z'
    );
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum MaybeRange {
    Range(std::ops::Range<i32>),
    Value(i32),
}

#[test]
fn test_range_untagged() {
    assert_eq!(
        ron::from_str::<MaybeRange>("0..5").unwrap(),
        MaybeRange::Range(0..5)
    );
    assert_eq!(
        ron::from_str::<MaybeRange>("42").unwrap(),
        MaybeRange::Value(42)
    );
}

#[test]
fn test_unclosed_ranges() {
    assert_eq!(ron::from_str::<std::ops::RangeTo<i32>>("..3").unwrap(), ..3);
    assert_eq!(
        ron::from_str::<std::ops::RangeFrom<i32>>("1..").unwrap(),
        1..
    );
}

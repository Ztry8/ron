use ron;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RangeTest {
    a: std::ops::Range<i32>,
    b: std::ops::RangeInclusive<i32>,
    c: std::ops::Range<f32>,
}

#[test]
fn test_ranges() {
    let de: RangeTest = ron::from_str(
        r#"
        (
            a: 1..3,
            b: 2..=5,
            c: 1.5..3.1,
        )
    "#,
    )
    .unwrap();

    dbg!(&de);
    dbg!(de.a.start, de.a.end);
    dbg!(de.b.start(), de.b.end());
    dbg!(de.c.start, de.c.end);
}

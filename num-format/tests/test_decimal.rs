#![cfg(feature = "with-decimal")]
mod common;

use num_format::{CustomFormat, Locale, ToFormattedString, WriteFormatted};
use rust_decimal::Decimal;

use crate::common::POLICIES;

#[test]
fn test_found_cases() {
    let x = Decimal::from_str_exact("14.98").unwrap();
    let format = Locale::en;

    assert_eq!(x.to_formatted_string(&format), "14.98");
}

#[test]
fn test_decimal() {
    let test_cases: &[(&str, Decimal, &CustomFormat)] = &[
        (
            "1.2340",
            Decimal::from_str_exact("1.2340").unwrap(),
            &POLICIES[0],
        ),
        (
            "1.2340",
            Decimal::from_str_exact("1.2340").unwrap(),
            &POLICIES[1],
        ),
        (
            "1.2340",
            Decimal::from_str_exact("1.2340").unwrap(),
            &POLICIES[2],
        ),
        (
            "1.2340",
            Decimal::from_str_exact("1.2340").unwrap(),
            &POLICIES[3],
        ),
        (
            "1.2340",
            Decimal::from_str_exact("1.2340").unwrap(),
            &POLICIES[4],
        ),
        ("555", Decimal::from_str_exact("555").unwrap(), &POLICIES[0]),
        ("555", Decimal::from_str_exact("555").unwrap(), &POLICIES[1]),
        ("555", Decimal::from_str_exact("555").unwrap(), &POLICIES[2]),
        ("555", Decimal::from_str_exact("555").unwrap(), &POLICIES[3]),
        ("555", Decimal::from_str_exact("555").unwrap(), &POLICIES[4]),
        (
            "222.000",
            Decimal::from_str_exact("222.000").unwrap(),
            &POLICIES[0],
        ),
        (
            "222.000",
            Decimal::from_str_exact("222.000").unwrap(),
            &POLICIES[1],
        ),
        (
            "222.000",
            Decimal::from_str_exact("222.000").unwrap(),
            &POLICIES[2],
        ),
        (
            "222.000",
            Decimal::from_str_exact("222.000").unwrap(),
            &POLICIES[3],
        ),
        (
            "222.000",
            Decimal::from_str_exact("222.000").unwrap(),
            &POLICIES[4],
        ),
        (
            "-1.2340",
            Decimal::from_str_exact("-1.2340").unwrap(),
            &POLICIES[0],
        ),
        (
            "\u{200e}-\u{200e}1.2340",
            Decimal::from_str_exact("-1.2340").unwrap(),
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}1.2340",
            Decimal::from_str_exact("-1.2340").unwrap(),
            &POLICIES[2],
        ),
        (
            "\u{200e}-\u{200e}1.2340",
            Decimal::from_str_exact("-1.2340").unwrap(),
            &POLICIES[3],
        ),
        (
            "\u{200e}-\u{200e}1.2340",
            Decimal::from_str_exact("-1.2340").unwrap(),
            &POLICIES[4],
        ),
        (
            "374,598,734,958,739.485034500000",
            Decimal::from_str_exact("374598734958739.485034500000").unwrap(),
            &POLICIES[0],
        ),
        (
            "374𠜱598𠜱734𠜱958𠜱739.485034500000",
            Decimal::from_str_exact("374598734958739.485034500000").unwrap(),
            &POLICIES[1],
        ),
        (
            "37𠜱45𠜱98𠜱73𠜱49𠜱58𠜱739.485034500000",
            Decimal::from_str_exact("374598734958739.485034500000").unwrap(),
            &POLICIES[2],
        ),
        (
            "374598734958739.485034500000",
            Decimal::from_str_exact("374598734958739.485034500000").unwrap(),
            &POLICIES[3],
        ),
        (
            "374598734958739.485034500000",
            Decimal::from_str_exact("374598734958739.485034500000").unwrap(),
            &POLICIES[4],
        ),
    ];

    for (expected, input, format) in test_cases {
        // ToFormattedString
        assert_eq!(expected.to_string(), input.to_formatted_string(*format));

        // WriteFormatted (io::Write)
        let mut v = Vec::new();
        v.write_formatted(input, *format).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert_eq!(expected.to_string(), s);

        // WriteFormatted (fmt::Write)
        let mut s = String::new();
        s.write_formatted(input, *format).unwrap();
        assert_eq!(expected.to_string(), s);
    }
}

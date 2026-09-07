use super::{DecimalWordNameError, decimal_string_to_snake_case, decimal_string_to_word_tokens};

#[test]
fn decimal_to_words_zero_test() {
    assert_eq!(decimal_string_to_snake_case("0").unwrap(), "zero");
    assert_eq!(decimal_string_to_snake_case("-0.000").unwrap(), "zero");
}

#[test]
fn decimal_to_words_fraction_test() {
    assert_eq!(decimal_string_to_snake_case("0.000000000000000001").unwrap(), "one_over_one_quintillion");
    assert_eq!(decimal_string_to_snake_case("0.0000000000000000006").unwrap(), "six_over_ten_quintillion");
}

#[test]
fn decimal_to_words_integer_test() {
    assert_eq!(decimal_string_to_snake_case("6000").unwrap(), "six_thousand");
    assert_eq!(
        decimal_string_to_snake_case("123456789").unwrap(),
        "one_hundred_twenty_three_million_four_hundred_fifty_six_thousand_seven_hundred_eighty_nine"
    );
}

#[test]
fn decimal_to_words_mixed_and_negative_test() {
    assert_eq!(
        decimal_string_to_snake_case("-12.34").unwrap(),
        "negative_ten_two_and_thirty_four_over_one_hundred"
    );
}

#[test]
fn decimal_to_words_scientific_test() {
    assert_eq!(decimal_string_to_snake_case("1e3").unwrap(), "one_thousand");
    assert_eq!(decimal_string_to_snake_case("1e-18").unwrap(), "one_over_one_quintillion");
}

#[test]
fn decimal_to_words_tokens_output_test() {
    let tokens = decimal_string_to_word_tokens("0.05").unwrap();
    assert_eq!(tokens, vec!["five", "over", "one", "hundred"]);
}

#[test]
fn decimal_to_words_magnitude_limit_test() {
    assert_eq!(decimal_string_to_snake_case("1e-44").unwrap(), "one_over_one_hundred_tredecillion");

    let error = decimal_string_to_snake_case("1e-45").unwrap_err();
    assert!(matches!(error, DecimalWordNameError::UnsupportedFractionalMagnitude { .. }));
}

#[test]
fn decimal_to_words_scalar_range_limit_test() {
    let max_positive = format!("4{}", "9".repeat(35));
    let beyond_positive = format!("5{}", "0".repeat(35));
    let min_negative = format!("-5{}", "9".repeat(35));
    let beyond_negative = format!("-6{}", "0".repeat(35));

    assert!(decimal_string_to_snake_case(&max_positive).is_ok());
    assert!(decimal_string_to_snake_case(&min_negative).is_ok());

    assert!(matches!(
        decimal_string_to_snake_case(&beyond_positive),
        Err(DecimalWordNameError::OutOfRepresentableRange)
    ));
    assert!(matches!(
        decimal_string_to_snake_case(&beyond_negative),
        Err(DecimalWordNameError::OutOfRepresentableRange)
    ));
}

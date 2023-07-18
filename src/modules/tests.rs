/*
LUHNY.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "validate_IMEI"
/// method from "./luhny.rs".
use super::luhny::validate_IMEI;

/// Importing the "is_number_sequence"
/// method from "./luhny.rs".
use super::luhny::is_number_sequence;

/// Tests the "is_number_sequence" method.
#[test]
pub fn test_is_number_sequence() -> () {
    let real_num_sequence: String = String::from("12345678");
    let fake_num_sequence: String = String::from("12345_78");
    assert_eq!(
        is_number_sequence(
            &real_num_sequence
        ), 
        true
    );
    assert_eq!(
        is_number_sequence(
            &fake_num_sequence
        ),
        false
    );
}

/// Tests the "validate_IMEI" method.
#[test]
pub fn test_validate_imei() -> () {
    let real_imei: String = String::from("353879234252633");
    let fake_imei: String = String::from("353879234252634");
    assert_eq!(
        validate_IMEI(
            &real_imei
        ), 
        true
    );
    assert_eq!(
        validate_IMEI(
            &fake_imei
        ), 
        false
    );
}
use check_digits::Solution;

#[test]
fn test_has_same_digits() {
    // Test case 1: All digits are already the same
    assert_eq!(Solution::has_same_digits("1111".to_string()), true);

    // Test case 2: Digits can be made equal
    assert_eq!(Solution::has_same_digits("242".to_string()), true);
    assert_eq!(Solution::has_same_digits("3902".to_string()), true);
    assert_eq!(Solution::has_same_digits("797".to_string()), true);

    // Test case 3: Digits cannot be made equal
    assert_eq!(Solution::has_same_digits("355".to_string()), false);
    assert_eq!(Solution::has_same_digits("34789".to_string()), false);

    // Test case 4: Edge cases
    assert_eq!(Solution::has_same_digits("".to_string()), true); // Empty string
    assert_eq!(Solution::has_same_digits("1".to_string()), true); // Single digit
    assert_eq!(Solution::has_same_digits("12".to_string()), false); // Two digits, not equal
    assert_eq!(Solution::has_same_digits("22".to_string()), true); // Two digits, equal
}
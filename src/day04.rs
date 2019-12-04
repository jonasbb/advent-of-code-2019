use std::ops::Range;

/// Iterate over the digits over number as writtin in base10
///
/// Given the input number return the digits from lowest to highest:
///
/// ```
/// Input: 123789
/// Iterator: 9, 8, 7, 3, 2, 1
/// ```
#[derive(Debug)]
struct DigitsIter {
    number: usize,
}

impl Iterator for DigitsIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            None
        } else {
            let digit = self.number % 10;
            self.number /= 10;
            Some(digit)
        }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Range<usize> {
    let mut parts = input.split('-').map(|part| part.parse().unwrap());
    let min = parts.next().unwrap();
    let max = parts.next().unwrap();
    min..max
}

#[aoc(day4, part1)]
fn part1(input: &Range<usize>) -> usize {
    // Since we iterate over the digits of the numbers in reverse order, we need to invert the condition.
    // This means digits only every decrease or stay the same, but they never increase
    input
        .clone()
        .filter(|&x| is_number_valid_for_part1(x))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Range<usize>) -> usize {
    // Since we iterate over the digits of the numbers in reverse order, we need to invert the condition.
    // This means digits only every decrease or stay the same, but they never increase
    input
        .clone()
        .filter(|&x| is_number_valid_for_part2(x))
        .count()
}

fn is_number_valid_for_part1(number: usize) -> bool {
    // Check if the number fulfills both conditions
    // 1. two adjacent digits are the same
    // 2. only decreasing numbers
    let mut last_digit = 10;
    let mut has_double_digit = false;
    let mut is_decreasing = true;
    (DigitsIter { number }).for_each(|digit| {
        if digit > last_digit {
            is_decreasing = false;
        }
        if digit == last_digit {
            has_double_digit = true;
        }
        last_digit = digit;
    });
    has_double_digit && is_decreasing
}

fn is_number_valid_for_part2(number: usize) -> bool {
    // Check if the number fulfills both conditions
    // 1. two adjacent digits are the same
    //     - but they are not part of a larger group
    // 2. only decreasing numbers
    let mut seconds_last_digit = 11;
    let mut last_digit = 10;
    let mut has_double_digit = false;
    let mut is_decreasing = true;
    let mut iter = (DigitsIter { number }).peekable();
    while let Some(digit) = iter.next() {
        if digit > last_digit {
            is_decreasing = false;
        }
        if digit == last_digit && !(last_digit == seconds_last_digit || Some(&digit) == iter.peek()) {
            has_double_digit = true;
        }
        seconds_last_digit = last_digit;
        last_digit = digit;
    };
    has_double_digit && is_decreasing
}

#[test]
fn test_digits_iter_1() {
    let mut iter = DigitsIter { number: 111111 };
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_digits_iter_2() {
    let mut iter = DigitsIter { number: 223450 };
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(5), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_digits_iter_3() {
    let mut iter = DigitsIter { number: 123789 };
    assert_eq!(Some(9), iter.next());
    assert_eq!(Some(8), iter.next());
    assert_eq!(Some(7), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_part1() {
    // 111111 meets these criteria (double 11, never decreases).
    assert_eq!(
        true,
        is_number_valid_for_part1(111111),
        "111111 should be true"
    );
    // 223450 does not meet these criteria (decreasing pair of digits 50).
    assert_eq!(
        false,
        is_number_valid_for_part1(223450),
        "223450 should be false"
    );
    // 123789 does not meet these criteria (no double)
    assert_eq!(
        false,
        is_number_valid_for_part1(123789),
        "123789 should be false"
    );
}

#[test]
fn test_part2() {
    // 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
    assert_eq!(
        true,
        is_number_valid_for_part2(112233),
        "112233 should be true"
    );
    // 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
    assert_eq!(
        false,
        is_number_valid_for_part2(123444),
        "123444 should be false"
    );
    // 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
    assert_eq!(
        true,
        is_number_valid_for_part2(111122),
        "111122 should be true"
    );
}

pub fn get_frequent_numbers_asc_strategy(numbers: &[u8]) -> Vec<u8> {
    // NOTE: Contracts:
    //       1) Elements are in the range [0..10]
    //       2) 30 elements in total... but this function can handle more

    // NOTE: 16 bits are enough for the range 11 elements
    let mut numbers_bitmask = 0u16;
    let mut frequent_numbers_bitmask = 0u16;

    for number in numbers.iter() {
        // NOTE: To prevent overflow
        debug_assert!(*number <= 10);

        let bit = 1u16 << number;
        let has_duplicates = (numbers_bitmask & bit) > 0;

        if has_duplicates {
            frequent_numbers_bitmask |= bit;
        } else {
            numbers_bitmask |= bit;
        }
    }

    let mut result = vec![];

    // NOTE: Since we iterate over numbers ascending, we do not need to do any sorting
    for number in 0..=10 {
        let bit = 1u16 << number;
        let is_frequent_number = (frequent_numbers_bitmask & bit) > 0;

        if is_frequent_number {
            result.push(number);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::get_frequent_numbers_asc_strategy;

    #[test]
    fn test_basic_cases() {
        assert_eq!(
            get_frequent_numbers_asc_strategy(&[3, 2, 5, 1, 5, 7, 2, 1]),
            vec![1, 2, 5]
        );
        assert_eq!(get_frequent_numbers_asc_strategy(&[5, 7, 7]), vec![7]);
    }

    #[test]
    fn test_no_duplicates() {
        let empty_vec: Vec<u8> = vec![];

        assert_eq!(get_frequent_numbers_asc_strategy(&[3, 2, 5, 1]), empty_vec);
    }

    #[test]
    fn test_many_duplicates() {
        assert_eq!(
            get_frequent_numbers_asc_strategy(&[7, 5, 5, 5, 5, 5, 4]),
            vec![5]
        );
    }

    #[test]
    fn test_all_duplicates() {
        assert_eq!(get_frequent_numbers_asc_strategy(&[0, 0]), vec![0]);
        assert_eq!(get_frequent_numbers_asc_strategy(&[1, 1, 1, 1, 1]), vec![1]);
    }
}

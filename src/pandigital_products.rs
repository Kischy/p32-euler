mod digits;
use digits::get_all_digits;

pub fn numbers_are_pandigital_in_total(nums: &[u128]) -> bool {
    let mut digits = get_all_digits(nums);
    digits.sort();
    let mut counter = 1;
    for digit in digits {
        if digit != counter {
            return false;
        }
        counter+=1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::numbers_are_pandigital_in_total;

    #[test]
    fn is_pandigital_number_tests() {
        assert_eq!(numbers_are_pandigital_in_total(&vec![1, 2, 3]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![1, 2, 3, 4]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![1]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![2]), false);
        assert_eq!(numbers_are_pandigital_in_total(&vec![2, 3, 4, 1]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![9, 1, 2, 4, 3, 5, 6, 7, 8]), true);
        assert_eq!(
            numbers_are_pandigital_in_total(&vec![9, 1, 2, 4, 3, 5, 6, 7, 8, 8]),
            false
        );
    }
}

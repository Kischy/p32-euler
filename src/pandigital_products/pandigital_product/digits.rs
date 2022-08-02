fn get_digits(num: u128) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn get_all_digits(nums: &[u128]) -> Vec<u32> {
    let mut digits = Vec::new();
    for ele in nums {
        digits.append(&mut get_digits(*ele));
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::get_all_digits;
    use super::get_digits;

    #[test]
    fn different_tests_of_get_digits() {
        assert_eq!(get_digits(0), vec![0]);
        assert_eq!(get_digits(123), vec![1, 2, 3]);
        assert_eq!(get_digits(5032), vec![5, 0, 3, 2]);
        assert_eq!(get_digits(9), vec![9]);
    }

    #[test]
    fn different_tests_of_get_all_digits() {
        assert_eq!(get_all_digits(&vec![0]), vec![0]);
        assert_eq!(get_all_digits(&vec![0, 123]), vec![0, 1, 2, 3]);
        assert_eq!(
            get_all_digits(&vec![0, 123, 5032]),
            vec![0, 1, 2, 3, 5, 0, 3, 2]
        );
        assert_eq!(
            get_all_digits(&vec![0, 123, 5032, 9]),
            vec![0, 1, 2, 3, 5, 0, 3, 2, 9]
        );
    }
}

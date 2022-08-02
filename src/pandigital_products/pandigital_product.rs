mod digits;

pub struct PandigitalProduct {
    multiplicand: u128,
    multiplier: u128,
    product: u128,
}

impl PandigitalProduct {
    pub fn new(multiplicand: u128, multiplier: u128, product: u128) -> PandigitalProduct {
        PandigitalProduct {
            multiplicand,
            multiplier,
            product,
        }
    }

    fn is_pandigital(&self) -> bool {
        return numbers_are_pandigital_in_total(&[
            self.multiplicand,
            self.multiplier,
            self.product,
        ]);
    }
}

fn numbers_are_pandigital_in_total(nums: &[u128]) -> bool {
    let mut digits = digits::get_all_digits(nums);
    digits.sort();
    let mut counter = 1;
    for digit in digits {
        if digit != counter {
            return false;
        }
        counter += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::numbers_are_pandigital_in_total;
    use super::PandigitalProduct;

    #[test]
    fn is_pandigital_number_tests() {
        assert_eq!(numbers_are_pandigital_in_total(&vec![1, 2, 3]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![1, 2, 3, 4]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![1]), true);
        assert_eq!(numbers_are_pandigital_in_total(&vec![2]), false);
        assert_eq!(numbers_are_pandigital_in_total(&vec![2, 3, 4, 1]), true);
        assert_eq!(
            numbers_are_pandigital_in_total(&vec![9, 1, 2, 4, 3, 5, 6, 7, 8]),
            true
        );
        assert_eq!(
            numbers_are_pandigital_in_total(&vec![9, 1, 2, 4, 3, 5, 6, 7, 8, 8]),
            false
        );
    }

    #[test]
    fn product_is_pandigital() {
        assert_eq!(PandigitalProduct::new(1, 2, 3).is_pandigital(), true);
        assert_eq!(PandigitalProduct::new(123, 456, 789).is_pandigital(), true);
        assert_eq!(PandigitalProduct::new(39, 186, 7254).is_pandigital(), true);
        assert_eq!(PandigitalProduct::new(39, 186, 72549).is_pandigital(), false);
    }
}

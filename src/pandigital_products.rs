mod pandigital_product;
use num::integer::Roots;
use pandigital_product::PandigitalProduct;

pub struct PandigitalProducts {
    product: u128,
}

impl PandigitalProducts {
    pub fn new(product: u128) -> PandigitalProducts {
        PandigitalProducts { product }
    }

    pub fn get_pandigital_products_with_nine_digits(&self) -> Vec<PandigitalProduct> {
        let mut pandigitals = Vec::new();

        for product in self.get_possible_products() {
            if product.is_pandigital() && product.has_nine_digits() {
                pandigitals.push(product);
            }
        }

        pandigitals
    }

    pub fn remove_duplicate_products(products: Vec<PandigitalProduct>) -> Vec<PandigitalProduct> {
        let mut new_products = Vec::new();

        for pr in products {
            if PandigitalProducts::contains(&new_products, &pr) == false {
                new_products.push(pr);
            }
        }

        new_products
    }

    fn contains(products: &Vec<PandigitalProduct>, product: &PandigitalProduct) -> bool {
        for pr in products {
            if pr.product_eq(&product) {
                return true;
            }
        }

        false
    }

    fn get_possible_products(&self) -> Vec<PandigitalProduct> {
        let mut products = Vec::new();

        for i in 2..self.product.sqrt() + 1 {
            if self.product % i == 0 {
                products.push(PandigitalProduct::new(i, self.product / i, self.product));
            }
        }

        products
    }
}

#[cfg(test)]
mod test {
    use super::PandigitalProduct;
    use super::PandigitalProducts;

    #[test]
    fn products_of_10() {
        let pp = PandigitalProducts::new(10);
        assert_eq!(
            pp.get_possible_products(),
            vec![PandigitalProduct::new(2, 5, 10),]
        )
    }

    #[test]
    fn products_of_50() {
        let pp = PandigitalProducts::new(50);
        assert_eq!(
            pp.get_possible_products(),
            vec![
                PandigitalProduct::new(2, 25, 50),
                PandigitalProduct::new(5, 10, 50),
            ]
        )
    }

    #[test]
    fn remove_duplicates() {
        assert_eq!(
            PandigitalProducts::remove_duplicate_products(vec![
                PandigitalProduct::new(2, 25, 50),
                PandigitalProduct::new(5, 10, 50),
                PandigitalProduct::new(10, 10, 100),
            ],),
            vec![
                PandigitalProduct::new(2, 25, 50),
                PandigitalProduct::new(10, 10, 100),
            ]
        )
    }
}

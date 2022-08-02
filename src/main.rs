use pandigital_products::PandigitalProducts;

mod pandigital_products;

fn main() {
    println!("Calculation started...\n\n");
    let mut answer_p32 = 0;

    let mut pandigital_products = Vec::new();

    for i in 1..10000 {
        pandigital_products
            .append(&mut PandigitalProducts::new(i).get_pandigital_products_with_nine_digits());
    }

    pandigital_products = PandigitalProducts::remove_duplicate_products(pandigital_products);

    print!("{:?}\n\n", pandigital_products);

    for pr in &pandigital_products {
        answer_p32 += pr.get_product();
    }

    println!(
        "The answer to problem 32 of project Euler is {}.",
        answer_p32
    );
}

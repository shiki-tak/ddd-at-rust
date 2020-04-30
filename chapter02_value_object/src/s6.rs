use std::fmt;

struct ModelNumber {
    product_code: String,
    branch: String,
    lot: String,
}

impl ModelNumber {
    fn new(product_code: &str, branch: &str, lot: &str) -> Self {
        Self {
            product_code: product_code.to_string(),
            branch: branch.to_string(),
            lot: lot.to_string(),
        }
    }
}

impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.product_code, self.branch, self.lot)
    }
}

#[test]
fn test_modelnumber() {
    let model_number = ModelNumber::new("1111", "abcdefg", "001");
    println!("{:?}", model_number);
}

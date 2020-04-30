use derive_getters::Getters;

#[derive(Clone, Debug, Getters, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

#[test]
fn test_full_name() {
    let name1 = FullName::new("taro", "yamada");
    let name2 = FullName::new("john", "smith");

    let name1_cloned = name1.clone();

    let compare1 = name1 == name1_cloned;
    let compare2 = name1 == name2;

    assert_ne!(name1, name2);
    assert_eq!(name1, name1.clone());
    assert!(compare1);
    assert!(!compare2);
}
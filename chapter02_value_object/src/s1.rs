#[derive(Clone, Debug)]
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

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
}

impl PartialEq for FullName {
    fn eq(&self, other: &Self) -> bool {
        self.first_name() == other.first_name() && self.last_name() == other.last_name()
    }
}

#[test]
fn test_full_name() {
    let name1 = FullName::new("taro", "yamada");
    let name2 = FullName::new("john", "smith");

    assert_ne!(name1, name2);
    assert_eq!(name1, name1.clone())
}
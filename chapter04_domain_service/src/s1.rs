use chapter03_entity::User;
use derive_new::new;

#[derive(Debug, new)]
pub struct UserService {}

impl UserService {
    pub fn exists(&self, _user: User) -> bool {
      //  unimplemented!();
        true
    }
}

#[test]
fn test_exists_user() {
    use chapter03_entity::UserId;

    let user_service = UserService::new();

    let user = User::new(UserId::new("id"), "Hoge".parse().unwrap());

    assert!(user_service.exists(user));
}


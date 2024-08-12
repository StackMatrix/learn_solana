use super::{entity::Model, repository::UserRepository};

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn get_user(&self, id: u32) -> Option<&Model> {
        self.repository.find_by_id(id)
    }
}
use super::entity::Model;

pub trait UserRepository {
    fn find_by_id(&self, id: u32) -> Option<&Model>;
    fn save(&mut self, user: Model);
}

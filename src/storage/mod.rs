use crate::model::build::Build;

pub trait Storage {
    fn get(&self, id: String) -> Result<Option<Build>, ()>;
}

pub struct Dummy;

impl Storage for Dummy {
    fn get(&self, _id: String) -> Result<Option<Build>, ()> {
        Ok(None)
    }
}

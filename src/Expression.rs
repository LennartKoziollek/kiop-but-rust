use crate::{Variable::Variable, Error::Error};

pub trait Expression<'a> {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Variable>, Error>;
    fn reduce(&self);
}
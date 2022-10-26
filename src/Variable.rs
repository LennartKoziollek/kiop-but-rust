use crate::{Expression::Expression, Error::Error};

#[derive(Debug)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl<'a> Expression<'a> for Variable {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Self>,Error> {
        Err(Error::FreeIdentifiers)
    }

    fn reduce(&self) {} //empty because you cant to anything to reduce vars
}

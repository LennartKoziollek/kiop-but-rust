use crate::{Expression::Expression, Error::Error};



struct Abstraction<'a> {
    header: String,
    body: &'a dyn Expression<'a>,
}

impl<'a> Expression<'a> for Abstraction<'a> {
    fn free_identifiers(&'a self) -> Result<Vec<&'a crate::Variable::Variable>, crate::Error::Error> {
        //here has to be actual logic
        Err(Error::FreeIdentifiers)
    }
    fn reduce(&self) {
       //actual logic here aswell 
    }
}
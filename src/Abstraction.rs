use std::fmt::Display;

use crate::{Expression::{Expression, ExpressionType}, Error::Error};



struct Abstraction<'a> {
    header: String,
    body: &'a dyn Expression<'a>, // do i really need pointers here? because this should be owned i think
}

impl<'a> Expression<'a> for Abstraction<'a> {
    fn free_identifiers(&'a self) -> Result<Vec<&'a crate::Variable::Variable>, crate::Error::Error> {
        //here has to be actual logic
        Err(Error::FreeIdentifiers)
    }
    fn reduce(&mut self) -> Result<(), Error> {
       Err(Error::CantReduceAbstraction)
    }

    fn get_type(&self) -> crate::Expression::ExpressionType {
        ExpressionType::Abstraction
    }
}

impl<'a> Abstraction<'a> {
    fn apply() {}
}

impl<'a> Display for Abstraction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Lambda{}.{})", self.header, self.body)
    }
}
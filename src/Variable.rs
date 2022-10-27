use std::fmt::Display;

use crate::{
    Error::Error,
    Expression::{Expression, ExpressionType},
};

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl<'a> Expression<'a> for Variable {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Self>, Error> {
        Err(Error::FreeIdentifiers)
    }

    fn reduce(&mut self) -> Result<(), Error>{
        Err(Error::CantReduceVariable)
    } //empty because you cant to anything to reduce vars

    fn get_type(&self) -> crate::Expression::ExpressionType {
        ExpressionType::Variable
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
use std::fmt::Display;

use crate::{Variable::Variable, Error::Error};

pub trait Expression<'a> : Display {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Variable>, Error>;
    fn reduce(&mut self) -> Result<(), Error>;
    fn get_type(&self) -> ExpressionType;
}

pub enum ExpressionType {
    Application,
    Abstraction,
    Variable
}
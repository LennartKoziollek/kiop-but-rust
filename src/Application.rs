use std::fmt::Display;

use crate::{
    Error::Error,
    Expression::{Expression, ExpressionType},
    Variable::Variable,
};

struct Application<'a> {
    left: &'a dyn Expression<'a>,
    right: &'a dyn Expression<'a>,
}

impl<'a> Expression<'a> for Application<'a> {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Variable>, Error> {
        let mut vec = self.left.free_identifiers()?;
        vec.append(&mut self.right.free_identifiers()?);
        Ok(vec)
    }

    fn reduce(&mut self) -> Result<(), Error> {
        if let ExpressionType::Abstraction = self.left.get_type() {
            // return ((Abstraction) left).apply(right);
            Ok(()) //TODO logic here
        } else {
            Err(Error::LeftNotExpression)
        }
    }

    fn get_type(&self) -> ExpressionType {
        ExpressionType::Application
    }
}


impl<'a> Display for Application<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.left, self.right)
    }
}

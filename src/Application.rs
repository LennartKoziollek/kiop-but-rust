use crate::{Error::Error, Expression::Expression, Variable::Variable};

//rust things here lol
struct Application<'a> {
    left: &'a dyn Expression<'a>,
    right: &'a dyn Expression<'a>,
}

impl<'a> Application<'a> {
    fn free_identifiers(&'a self) -> Result<Vec<&'a Variable>, Error> {
        Err(Error::FreeIdentifiers)
    }

    fn reduce(&self) {}
}

impl<'a> Application<'a> {
    pub fn apply() {

    }
}
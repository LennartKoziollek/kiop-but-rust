use crate::Expression::Expression;

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
    fn free_identifiers(&'a self) -> Vec<&'a Self> {
        let free_identifiers: Vec<&Variable> = vec![self];
        free_identifiers
    }

    fn reduce(&self) {} //empty because you cant to anything to reduce vars
}

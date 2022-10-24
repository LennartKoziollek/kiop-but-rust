use crate::Variable::Variable;

pub trait Expression<'a> {
    fn free_identifiers(&'a self) -> Vec<&'a Variable>;
    fn reduce(&self);
}
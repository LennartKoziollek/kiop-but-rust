#[derive(Debug)]
pub enum Error {
    FreeIdentifiers,
    LeftNotExpression,
    CantReduceVariable,
    CantReduceAbstraction,
}
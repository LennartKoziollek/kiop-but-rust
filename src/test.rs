use crate::{Expression::Expression, Variable::Variable};

pub fn test() {
    let test;
    {
        let v = Variable::new(String::from("x"));
        test = v.free_identifiers();
        println!("{:?}", test);
    }
}

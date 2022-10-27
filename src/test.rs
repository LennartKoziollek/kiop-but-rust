use crate::{Expression::Expression, Variable::Variable};

pub fn test() {
    {
        let v = Variable::new(String::from("x"));
        println!("{}", v);
    }
}

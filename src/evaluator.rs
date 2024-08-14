
pub mod ast {
    pub enum Stmt{
        Expr(Expr),
        Let(Name,Expr),
        //Binery(BInaryExpr),
    }
    pub struct Name{
       pub value:String
    }
    pub enum Expr {
        Literal(String),
        IntLit(i64),

    }
}
pub mod visit {
    use crate::parse;
    use crate::token;
    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, e: &parse::Expr) -> T;


    }

    pub struct Evaluator;
    impl Evaluator {
        

    pub fn evaluate(expression: parse::Expr) -> String  {
        match expression {
        parse::Expr::Literal(literal) => literal.to_owned(),
            parse::Expr::IntLit(int_val) => int_val.to_string(),
    }

}
    }
}

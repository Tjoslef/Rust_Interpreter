
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
        FloatLit(f64),

    }
}
pub mod visit {
    use crate::parse;
    use crate::parse::Expr;
    use crate::token;
    use crate::token::LiteralTokenType::STRING;

    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, e: Option<(Expr)>) -> T;


    }

    pub struct Evaluator;
    impl Evaluator {
        

    pub fn evaluate(expression: Option<Expr> ) -> String  {
        match expression {
        Some(Expr::Literal(literal)) => literal.to_owned(),
            Some(Expr::IntLit(int_val)) => int_val.to_string(),
            Some(Expr::FloatLit(float_val)) => float_val.to_string(),
            Some(Expr::BoolLite(bool_val)) => bool_val.to_string(),
            _=>{
               let mut ups = String::new();
                ups.push_str("error in evaluate");
                ups
            }
    }

}
    }
}

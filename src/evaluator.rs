
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
        //Binary(Box<BinaryExpr>),

    }
    //pub struct BinaryExpr {
      //  pub left: Expr,
        //pub op: TokenType,
      //  pub right: Expr,
   // }
}
pub mod visit {
    use crate::parse::Expr;
    use crate::token::{SymbolTokenType, TokenType};

    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, e: Option<(Expr)>) -> T;
    }

    pub struct Evaluator;
    impl Evaluator {
        pub fn evaluate(expression: Option<Expr>) -> String {
            match expression {
                Some(Expr::Literal(literal)) => literal.to_owned(),
                Some(Expr::IntLit(int_val)) => int_val.to_string(),
                Some(Expr::FloatLit(float_val)) => float_val.to_string(),
                Some(Expr::BoolLite(bool_val)) => bool_val.to_string(),
                Some(Expr::Mult(float_val1)) => float_val1.to_string(),
                Some(Expr::Binary(binary_expr)) => {
                    println!("evaluator reach");
                    let left = Evaluator::evaluate(Some(binary_expr.left)); // Pass the left side as an `Option<Expr>`
                    let right = Evaluator::evaluate(Some(binary_expr.right)); // Pass the right side as an `Option<Expr>`

                    // Convert strings to numbers for arithmetic operations
                    let left_num = left.parse::<f64>().unwrap_or(0.0);
                    let right_num = right.parse::<f64>().unwrap_or(0.0);

                    match binary_expr.op {
                        SymbolTokenType::STAR => (left_num * right_num).to_string(),
                        SymbolTokenType::SLASH => {
                            if right_num == 0.0 {
                                panic!("Division by zero is not allowed."); // Handle division by zero
                            }
                            println!("{}{}",left_num,right_num);
                            (left_num / right_num).to_string()

                        },
                        _ => "Unsupported operation".to_string(), // Handle other operations
                    }
                }
                _ => "Error in evaluation".to_string(), // Catch-all error message
            }
        }
    }
}
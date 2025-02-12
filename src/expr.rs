pub mod expr {
use crate::token_type::Token;

trait ExpressionBehaviors {
  fn interpret(&self) -> ();
  fn resolve(&self) -> ();
  fn analyze(&self) -> ();
}

pub struct Binary {
    pub left: String,
    pub operator: Token<String>,
    pub right: String,
}
pub struct Grouping {
    pub expression: String,
}
pub struct Literal<T> {
    pub value: Option<T>,
}
pub struct Unary {
    pub operator: Token<String>,
    pub right: String,
}
impl ExpressionBehaviors for Binary { 
  fn interpret(&self) -> () {
                        ()
                    }
  fn resolve(&self) -> () {
                        ()
                    }
  fn analyze(&self) -> () {
                        ()
                    }
}
impl ExpressionBehaviors for Grouping { 
  fn interpret(&self) -> () {
                        ()
                    }
  fn resolve(&self) -> () {
                        ()
                    }
  fn analyze(&self) -> () {
                        ()
                    }
}
impl ExpressionBehaviors for Literal<T> { 
  fn interpret(&self) -> () {
                        ()
                    }
  fn resolve(&self) -> () {
                        ()
                    }
  fn analyze(&self) -> () {
                        ()
                    }
}
impl ExpressionBehaviors for Unary { 
  fn interpret(&self) -> () {
                        ()
                    }
  fn resolve(&self) -> () {
                        ()
                    }
  fn analyze(&self) -> () {
                        ()
                    }
}
    pub enum Visitor<T> {    VisitBinary(Binary),    VisitGrouping(Grouping),    VisitLiteral<T>(Literal<T>),    VisitUnary(Unary),}}


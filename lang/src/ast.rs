pub enum Expr {
    Term(Term),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Num(Num),
    Str(String),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq)]
pub enum Num {
    Int(i32),
    Double(f64),
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub id: i32,
    pub name: String,
}

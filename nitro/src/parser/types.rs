#[derive(Debug, Clone)]
pub enum ConstExpr {
    Bool(bool),
    Int(i64),
    Real(f64),
    Char(char),
    String(String)
}

#[derive(Debug, Clone)]
pub enum ConstExpr {
    Bool(bool),
    Int(i64),
    Real(f64),
    Char(char),
    String(String)
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Any,
    Con,
    Ident { id: String },
    Tagged { pat: Box<Pattern> },
    Record { fields: Vec<(String, Pattern)> },
    Named {
        pat: Box<Pattern>,
        name: String
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var { var: String },
    New { expr: Box<Expr> },
    Const { expr: ConstExpr },
    Parens { expr: Box<Expr> },
    Match { rules: Vec<(Pattern, Expr)> },
    NewRecord { fields: Vec<(String, Expr)> },
    Abstraction {
        pat: Pattern,
        arg: Box<Expr>
    },
    Application {
        func: Box<Expr>,
        arg: Box<Expr>
    },
    GetField {
        expr: Box<Expr>,
        name: String
    },
    SetField {
        expr: Box<Expr>,
        name: String,
        value: Box<Expr>
    },
    Let {
        pat: Pattern,
        value: Box<Expr>,
        expr: Box<Expr>
    },
    Recursion {
        var: String,
        pat: Pattern,
        value: Box<Expr>,
        expr: Box<Expr>
    }
}

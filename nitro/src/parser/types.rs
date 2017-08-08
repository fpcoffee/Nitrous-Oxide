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
    Const { constant: ConstExpr },
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
    Const { constant: ConstExpr },
    New { expr: Box<Expr> },
    Parens { expr: Box<Expr> },
    NewRecord { fields: Vec<(String, Expr)> },
    Match {
        expr: Box<Expr>,
        rules: Vec<(Pattern, Expr)>
    },
    Abstraction {
        pat: Pattern,
        expr: Box<Expr>
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

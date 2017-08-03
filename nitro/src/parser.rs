pub enum Lambda<'a> {
    Variable { expr: String },
    Function { var: String, expr: &'a Lambda<'a> },
    Application { func: &'a Lambda<'a>, arg: &'a Lambda<'a> }
}

pub fn show_lambda(term: &Lambda) -> String {
    match *term {
        Lambda::Variable { ref expr } => expr.clone(),
        Lambda::Function { ref var, ref expr } =>
            format!("(λ{}.{})", var, show_lambda(expr)),
        Lambda::Application { ref func, ref arg } =>
            format!("({} {})", show_lambda(func), show_lambda(arg)),
    }
}

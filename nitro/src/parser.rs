use std::str;
use lambda;

#[derive(Debug, Clone)]
pub enum Lambda {
    Variable { var: String },
    Function { var: String, expr: Box<Lambda> },
    Application { func: Box<Lambda>, arg: Box<Lambda> }
}

pub fn show_lambda(term: &Lambda) -> String {
    match *term {
        Lambda::Variable { ref var } => var.clone(),
        Lambda::Function { ref var, ref expr } =>
            format!("(λ{}.{})", var, show_lambda(expr)),
        Lambda::Application { ref func, ref arg } =>
            format!("({} {})", show_lambda(func), show_lambda(arg)),
    }
}

pub fn read_lambda(input: &str) -> Option<Lambda> {
    let result = lambda::parse_Term(input);

    if result.is_err() {
        println!("{:?}", result);
    }

    result.ok()
}

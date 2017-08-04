use std::str;
use nom::{IResult,alpha,space};

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
    let result = lambda_term(input.as_bytes());

    if result.is_done() {
        Some(result.unwrap().1)
    } else {
        None
    }
}

fn lambda_term(input: &[u8]) -> IResult<&[u8], Lambda> {
    alt!(input, variable | function | application)
}

fn decode(input: &[u8]) -> String {
    str::from_utf8(input).unwrap().to_string()
}

fn variable(input: &[u8]) -> IResult<&[u8], Lambda> {
    do_parse!(input,
        var: alpha >>
        (Lambda::Variable {var: decode(var)})
    )
}

fn function(input: &[u8]) -> IResult<&[u8], Lambda> {
    delimited!(input,
               tag!("("),
               do_parse!(
                   tag!("λ") >>
                   var: alpha >>
                   tag!(".") >>
                   expr: lambda_term >>
                   (Lambda::Function {
                       var: decode(var),
                       expr: Box::new(expr)
                   })
               ),
               tag!(")")
    )
}

fn application(input: &[u8]) -> IResult<&[u8], Lambda> {
    delimited!(input,
               tag!("("),
               do_parse!(
                   func: lambda_term >>
                   call!(space) >>
                   arg: lambda_term >>
                   (Lambda::Application {
                       func: Box::new(func),
                       arg: Box::new(arg)
                   })
               ),
               tag!(")")
        )
}

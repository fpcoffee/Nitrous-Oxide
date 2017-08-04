extern crate nitro;

use nitro::parser;
use nitro::parser::Lambda;

fn main() {
    let xxyx = Lambda::Variable {var: "xxyx".to_string()};
    let yf = Lambda::Function {var: "y".to_string(), expr: Box::new(xxyx)};
    let xf = Lambda::Function {var: "x".to_string(), expr: Box::new(yf)};
    let abc = Lambda::Variable {var: "abc".to_string()};
    let ap = Lambda::Application {func: Box::new(xf), arg: Box::new(abc)};
    println!("{}", parser::show_lambda(&ap));

    let term = "((λx.(λy.xxyx)) abc)";
    println!("{}", term);

    let parsed = parser::read_lambda(term);
    println!("{}", parser::show_lambda(&parsed.unwrap()));
}

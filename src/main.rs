extern crate nitro;

use nitro::parser;
use nitro::parser::Lambda;

fn main() {
    let two = Lambda::Variable { expr: "2".to_string() };
    let x = Lambda::Variable { expr: "x + x".to_string() };
    let f = Lambda::Function { var: "x".to_string(), expr: &x };
    let ap = Lambda::Application { func: &f, arg: &two};

    // println!("{:?}", parser::show_lambda(two));
    println!("{}", parser::show_lambda(&ap));
}

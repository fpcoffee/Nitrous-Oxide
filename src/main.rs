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

    let zero = "(λf.(λx.x))";
    let one = "(λf.(λx.(f x)))";
    let succ = "(λn.(λf.(λx.((n f) x))))";
    let plus = "(λm.(λn.(λf.(λx.((m f) ((n f) x))))))";
    let fix = "(λf.((λx.(f (x x))) (λx.(f (x x)))))";
    let pred = "(λn.(λf.(λx.(((n (λg.(λh.(h (g f))))) (λu.x)) (λu.u)))))";
    let plustwo = format!("(λk.({succ} ({succ} k)))", succ=succ);

    for term in vec![zero,one,succ,plus,fix,pred,&plustwo[..]] {
        let parsed = parser::read_lambda(term);

        match parsed {
            Some(lambda) => {
                println!("{}", term);
                println!("{:#?}\n", lambda);

            },
            None => println!("!!! {}", term)
        }
    }
}

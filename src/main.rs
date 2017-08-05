extern crate nitro;

use nitro::parser::expr;

fn main() {
    let reals = vec![
        "3.14", "-3.14", "2.5E10", "2.5e10", "2.5E-10", "5.", ".5", "0.5",
        "inf", "-inf", "NaN", "."
    ];

    for r in reals.iter() {
        println!("{}", r);
        println!("{:?}\n", expr::parse_Real(r));
    }
}

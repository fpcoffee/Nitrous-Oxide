extern crate nitro;

use nitro::parser::expr;

fn main() {
    let reals = vec![
        "3.14", "-3.14", "2.5E10", "2.5e10", "2.5E-10", "5.", ".5", "0.5",
        "inf", "-inf", "NaN", "."
    ];

    for r in reals.iter() {
        println!("{}", r);
        println!("{:?}\n", expr::parse_ConstExpr(r));
    }

    let chars = vec![
        "'\\0'", "'\\a'", "'\\b'", "'\\t'", "'\\n'", "'\\v'", "'\\f'", "'\\r'",
        "'\\u1337'", "'\\x42'", "'a'", "'s'", "'d'", "'f'", "'\x7f'"
    ];

    for c in chars.iter() {
        println!("{}", c);
        println!("{:?}\n", expr::parse_ConstExpr(c));
    }

    let strs = vec![
        "\"\"", "\"hello\"",  "\"Good\\x42ye!\"", "\"\\\"Nevermore\\\"\"",
        "\"\\u03bb\\u03c7 \\u2192 \\u03c7\\u00b2\"",
        "\"Can a string be really long\\n\\tAnd have a new line and shit?\"",
        "\"This should fail\nyo\"", "\"Escaping something I shouldn\\'t\""
    ];

    for s in strs.iter() {
        println!("{}", s);
        println!("{:?}\n", expr::parse_ConstExpr(s));
    }
}

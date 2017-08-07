extern crate nitro;

use nitro::parser::expr;
use nitro::parser::types::ConstExpr;

#[test]
fn parse_reals() {
    let valid = vec![
        "inf", "-inf", "NaN", "-NaN", "0.123", "-0.123", "3.14", "-3.14", "0.",
        "-0.", "5.", "-5.", "0.123e2", "0.456E4", "0.789e-8", "0.101112E-16",
        "3.14e10", "-3.14e10", "0.e10", "-0.e10", "5.e10", "-5.e10", "123e45",
        "678E9", "10e-11", "12E-14", "0e12"
    ];

    for r in valid.iter() {
        let res = expr::parse_ConstExpr(r);

        assert!(
            match res {
                Ok(ConstExpr::Real(_)) => true,
                _ => false
            },
            format!("Cannot parse as real: \"{}\" -> {:?}", r, res)
        );
    }
}


#[test]
fn parse_bad_reals() {
    let invalid = vec![
        // Commented out tests can be incompletely parsed?
        "", ".", ".e10", "25", "3 .14", // "3..14", "3.14.15"
    ];

    for r in invalid.iter() {
        let res = expr::parse_ConstExpr(r);

        assert!(
            match res {
                Ok(ConstExpr::Real(_)) => false,
                _ => true
            },
            format!("Erroneously parsed real: \"{}\"", r)
        );
    }
}


#[test]
fn parse_chars() {
    let valid = vec![
        "'\\0'", "'\\a'", "'\\b'", "'\\t'", "'\\n'", "'\\v'", "'\\f'", "'\\r'",
        "'\\u1337'", "'\\x42'", "'a'", "'s'", "'d'", "'f'"
    ];

    for c in valid.iter() {
        let res = expr::parse_ConstExpr(c);
        assert!(
            match res {
                Ok(ConstExpr::Char(_)) => true,
                _ => false
            },
            format!("Cannot parse as char: \"{}\" -> {:?}", c, res)
        );
    }
}


#[test]
fn parse_strings() {
    let valid = vec![
        "\"\"", "\"hello\"",  "\"Good\\x42ye!\"", "\"\\\"Nevermore\\\"\"",
        "\"\\u03bb\\u03c7 \\u2192 \\u03c7\\u00b2\"",
        "\"Can a string be really long\\n\\tAnd escape a new line and stuff?\"",
    ];

    for s in valid.iter() {
        let res = expr::parse_ConstExpr(s);

        assert!(
            match res {
                Ok(ConstExpr::String(_)) => true,
                _ => false
            },
            format!("Cannot parse as string: \"{}\" -> {:?}", s, res)
        );
    }
}


#[test]
fn parse_bad_string() {
    let invalid = vec![
        "Unquoted?", "No open quote?\"", "\"No closing quote?",
        "\"Control\ncharacters\"", "\"Escaping something I shouldn\\'t\""
    ];

    for s in invalid.iter() {
        let res = expr::parse_ConstExpr(s);

        assert!(
            match res {
                Ok(ConstExpr::String(_)) => false,
                _ => true
            },
            format!("Erroneously parsed string: \"{}\" -> {:?}", s, res)
        );
    }
}

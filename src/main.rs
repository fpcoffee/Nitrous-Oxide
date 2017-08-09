extern crate nitro;

use std::io;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use nitro::parser::expr;

fn main() {
    let stdin = BufReader::new(io::stdin());

    print!("nitro>>> ");
    let _ = io::stdout().flush();

    for line in stdin.lines() {
        let input = line.unwrap();
        let res = expr::parse_Expr(&input[..]);

        println!("{:#?}\n", res);
        print!("nitro>>> ");
        let _ = io::stdout().flush();
    }
}

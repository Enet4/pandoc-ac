#![warn(rust_2018_idioms)]

use failure::Error;
use std::io::{self, Read};
use pandoc_ac::make_acronym_formatting;

fn main() -> Result<(), Error> {
    let json = {
        let mut stdin = io::stdin();
        let mut json = String::new();
        stdin.read_to_string(&mut json)?;
        json
    };
    let out = pandoc_ast::filter(json, make_acronym_formatting);
    print!("{}", out);

    Ok(())
}

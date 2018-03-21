#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use failure::Error;

mod options;
use options::*;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("ERROR: {}", err);
            ::std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let options = Options::from_args()?;
    println!("{:#?}", options);
    Ok(())
}

#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

extern crate git2;

use failure::Error;

mod diff;
mod options;
mod types;

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
    let repo = git2::Repository::open_from_env()?;

    println!("{:#?}", options);
    match options.tests_type {
        Type::RSpec => types::rspec(options, &repo),
    }
}

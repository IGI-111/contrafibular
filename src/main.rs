extern crate colored;
#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate itertools;
extern crate rand;
extern crate termion;

mod error;
mod field;
mod state;

use clap::{App, Arg};
use error::Result;
use field::Field;
use state::State;
use std::fs::File;
use std::io::prelude::*;

quick_main!(run);
fn run() -> Result<()> {
    let matches = App::new("Contrafibular")
        .about("An anaspeptic Befunge interpreter for all your frasmotic pericombobulations.")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Display the state of the program at every step"),
        )
        .arg(
            Arg::with_name("filename")
                .required(true)
                .help("Input program."),
        )
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let mut f = File::open(filename)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let mut state = State::with_field(Field::from_bin(&data));

    if matches.is_present("debug") {
        state.run_debug()
    } else {
        state.run()
    }?;

    Ok(())
}

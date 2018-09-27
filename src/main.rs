extern crate colored;
#[macro_use]
extern crate quick_error;
extern crate rand;

mod field;
mod instruction;
mod state;

use field::Field;
use state::State;

fn main() {
    let hello_world = ">              v\n\
                       v  ,,,,,\"Hello\"<\n\
                       >48*,          v\n\
                       v,,,,,,\"World!\"<\n\
                       >25*,@";
    let mut state = State::with_field(Field::from_str(hello_world));
    state.run().unwrap();
}

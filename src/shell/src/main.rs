pub mod lib;

use crate::lib::{run, setup};

fn main() {
    setup(".shellrc").expect("Error on shell setup");
    run();
}

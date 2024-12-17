use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/runner.rs"));

pub fn run_all(inputs: &HashMap<usize, String>) {
    run(inputs);
}

pub fn get_inputs() -> HashMap<usize, String> {
    inputs()
}

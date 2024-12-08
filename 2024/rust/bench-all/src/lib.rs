include!(concat!(env!("OUT_DIR"), "/runner.rs"));

pub fn run_all(inputs: &[String]) {
    run(inputs);
}

pub fn get_inputs() -> Vec<String> {
    inputs()
}

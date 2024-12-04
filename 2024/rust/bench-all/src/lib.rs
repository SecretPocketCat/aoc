use aoc_client::get_input;
use seq_macro::seq;

seq!(N in 1..=4 {
#(
    use day_~N;
)*

pub fn run_all(inputs: &[String]) {
#(
    let input = &inputs[N - 1];
    day_~N::solution::part_a(&input).expect("Valid result");
    day_~N::solution::part_b(&input).expect("Valid result");
)*
}

pub fn get_inputs() -> Vec<String> {
    let root = std::fs::canonicalize("..").expect("Parent dir");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut inputs = Vec::new();
#(
        let input = get_input(root.clone(), N).await.expect("Get all inputs");
        inputs.push(input);
)*
         inputs
     })
}
});

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use cargo_toml::{Dependency, DependencyDetail, Manifest};
use toml;

const DAY_CRATE_PREFIX: &str = "day-";

struct AoCDay {
    name: String,
    code_name: String,
    num: u8,
}
impl AoCDay {
    fn new(path: &str) -> Self {
        let num = path
            .chars()
            .skip(DAY_CRATE_PREFIX.len())
            .collect::<String>()
            .parse()
            .expect("Parsed day#");

        Self {
            name: path.to_string(),
            code_name: path.replace('-', "_"),
            num,
        }
    }
}

fn main() {
    // find days
    let root = fs::canonicalize("..").expect("Get workspace root");
    let days: Vec<_> = root
        .read_dir()
        .expect("Read src")
        .flatten()
        .skip(1)
        .filter_map(|e| {
            e.file_name().to_str().and_then(|p| {
                if p.starts_with(DAY_CRATE_PREFIX) {
                    Some(AoCDay::new(p))
                } else {
                    None
                }
            })
        })
        .collect();
    if days.is_empty() {
        return;
    }
    // update Cargo.toml
    let manifest_path: PathBuf = env::var("CARGO_MANIFEST_PATH").unwrap().into();
    let mut manifest = Manifest::from_path(&manifest_path).expect("Valid Cargo.toml");
    for day in &days {
        manifest.dependencies.insert(
            day.name.clone(),
            Dependency::Detailed(
                DependencyDetail {
                    path: Some(format!("../{}", day.name)),
                    ..Default::default()
                }
                .into(),
            ),
        );
    }
    let cargo_toml = toml::ser::to_string(&manifest).expect("Serialized Cargo.toml");
    fs::write(&manifest_path, &cargo_toml).expect(&format!(
        "Updated Cargo.toml written to '{manifest_path:?}'"
    ));
    // solutions runner code gen
    let code_run = days
        .iter()
        .map(|d| {
            let num = d.num;
            let name = &d.code_name;
            format!(
                r#"
    let input = &inputs[&{num}];
    {name}::solution::part_a(&input).expect("Valid result for {name} part a");
    {name}::solution::part_b(&input).expect("Valid result for {name} part b");
"#,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let code_get_input = days
        .iter()
        .map(|d| {
            let num = d.num;
            let name = &d.code_name;
            format!(
                r#"
    let input = aoc_client::get_input(root.clone(), {num}).await.expect("Get input for {name}");
    inputs.insert({num}, input);
"#,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let runner_rs = format!(
        r#"
pub fn run(inputs: &HashMap<usize, String>) {{
{code_run}
}}

pub fn inputs() -> HashMap<usize, String> {{
    let root = std::fs::canonicalize("..").expect("Parent dir");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {{
        let mut inputs = HashMap::new();
        {code_get_input}
        inputs
     }})
}}
"#
    );
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let runner_path = Path::new(&out_dir).join("runner.rs");
    fs::write(&runner_path, &runner_rs)
        .expect(&format!("Updated lib.rs written to '{runner_rs:?}'"));
}

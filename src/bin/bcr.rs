use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use batch_command_runner::job_collector::*;
use batch_command_runner::job_runner::single::run;
use batch_command_runner::common::parsers::parse_key_val;

#[derive(Debug, Parser)]
struct Args {
    match_regex: String,
    command: String,
    #[clap(short = 'D', value_parser = parse_key_val::<String, String>, number_of_values = 1)]
    defines: Vec<(String, String)>,
}

/// Convert our vec key/value list to hash map
///
/// TODO: gotta be a more rusty way to do this
fn to_hashmap(var_map: &Vec<(String, String)>) -> Result<HashMap<String, String>> {
    let mut ret = HashMap::new();

    for (k, v) in var_map {
        ret.insert(k.clone(), v.clone());
    }

    Ok(ret)
}

fn main() -> Result<()> {
    let opts = Args::parse();

    let var_map = to_hashmap(&opts.defines)?;

    let jobs = collect_jobs(
        opts.match_regex.as_str(),
        ".",
        opts.command.as_str(),
        &var_map,
    )?;

    run(&jobs)
}

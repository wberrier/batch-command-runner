use anyhow::Result;
use std::collections::HashMap;
use structopt::StructOpt;

mod job_collector;
use job_collector::*;

mod job_runner;
use job_runner::single::run;

mod common;
use common::parsers::parse_key_val;

#[derive(Debug, StructOpt)]
struct Args {
    match_regex: String,
    command: String,
    #[structopt(short = "D", parse(try_from_str = parse_key_val), number_of_values = 1)]
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
    let opts = Args::from_args();

    let var_map = to_hashmap(&opts.defines)?;

    let jobs = collect_jobs(
        opts.match_regex.as_str(),
        ".",
        opts.command.as_str(),
        &var_map,
    )?;

    run(&jobs)
}

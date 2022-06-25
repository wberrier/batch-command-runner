use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// Replace several keywords with computed values
/// {} -> input file
/// @VAR@ -> computed value
fn interpolate_variables(
    input_file: &str,
    command: &str,
    var_map: &HashMap<String, String>,
) -> Result<String> {
    let mut ret = command.to_string().replace("{}", input_file);

    // Populate some defines
    let mut var_map = var_map.clone();

    let file_path = Path::new(input_file);

    var_map.insert(
        "dirname".to_string(),
        // Better way?  .display() seems odd...
        file_path.parent().context("ERROR")?.display().to_string(),
    );

    if let Some(filenamebase) = file_path.file_stem() {
        var_map.insert(
            "filenamebase".to_string(),
            filenamebase.to_string_lossy().into_owned(),
        );
    }

    // Replace all defines
    for (k, v) in var_map {
        ret = ret.to_string().replace(
            // Formulate @key@
            format!("@{}@", k).as_str(),
            &v,
        );
    }

    Ok(ret)
}

/// Get List of files matching the regex
fn collect_files(match_regex: &str, directory: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    let re = Regex::new(match_regex)?;

    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        // NOTE: don't use filter_entry() because the starting directory is a directory,
        // which will get filtered out
        if !path.is_dir() {
            let path_str = path.to_str().context("Invalid path")?;

            // Check for regex match
            if re.is_match(path_str) {
                files.push(path_str.into());
            }
        }
    }
    Ok(files)
}

pub fn collect_jobs(
    match_regex: &str,
    directory: &str,
    command: &str,
    var_map: &HashMap<String, String>,
) -> Result<Vec<String>> {
    let mut jobs = Vec::new();

    for f in collect_files(match_regex, directory)? {
        jobs.push(interpolate_variables(f.as_str(), command, var_map)?);
    }

    Ok(jobs)
}

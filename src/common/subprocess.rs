use anyhow::Result;
use run_script::run_script;

pub fn getstatusoutput(command: &str) -> Result<(i32, String)> {
    // TODO: does this work to get stdout and stderr?
    let (code, output, _) = run_script!(format!("{} 2>&1", command))?;

    Ok((code, output))
}

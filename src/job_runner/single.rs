use anyhow::{bail, Result};

use crate::common::subprocess::getstatusoutput;

pub fn run(jobs: &Vec<String>) -> Result<()> {
    for j in jobs {
        println!("Executing: {}", j);
        let (code, output) = getstatusoutput(j)?;

        if code != 0 {
            bail!("Error running job: {}", j);
        }

        println!("{}", output);
    }
    Ok(())
}

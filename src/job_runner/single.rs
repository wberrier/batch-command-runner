use anyhow::Result;

use shleazy::getoutput_shell_or_err;

pub fn run(jobs: &Vec<String>) -> Result<()> {
    for j in jobs {
        println!("Executing: {}", j);
        let output = getoutput_shell_or_err(j)?;

        println!("{:?}", output);
    }
    Ok(())
}

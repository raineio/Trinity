use std::path::Path;

use anyhow::bail;
use clap::Parser;

#[derive(Parser)]
pub fn Init() {
    
}

impl Init {
    pub fn execute_init_command(&self) -> anyhow::Result<()> {
        let project_path = Path::new();

        match get_project_path() {
            Some(_) => bail!(
                "There already exists a Trinity project in this directory!"
            ),
        }
    }
}
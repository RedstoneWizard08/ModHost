use crate::add::add_crate;
use anyhow::Result;
use clap::{Parser, Subcommand};
use itertools::Itertools;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Add(AddCommand),
}

#[derive(Debug, Clone, Parser)]
pub struct AddCommand {
    pub crate_name: String,

    #[arg(short = 'V', long)]
    pub crate_version: Option<String>,

    #[arg(short = 'F', long)]
    pub features: Vec<String>,

    #[arg(short = 'p', long)]
    pub pkg: String,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            Commands::Add(add) => {
                if add.crate_name.contains("@") && add.crate_version.is_none() {
                    let (name, ver) = add
                        .crate_name
                        .split("@")
                        .map(|v| v.to_string())
                        .collect_tuple()
                        .unwrap();

                    add_crate(name, Some(ver), add.features, add.pkg).await?;
                } else {
                    let name = add.crate_name.split("@").next().unwrap().to_string();

                    add_crate(name, add.crate_version, add.features, add.pkg).await?;
                }
            }
        }

        Ok(())
    }
}

use crate::{add::add_crate, rm::remove_crate};
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
    Remove(RemoveCommand),
}

#[derive(Debug, Clone, Parser)]
pub struct AddCommand {
    pub crate_name: String,

    #[arg(short = 'F', long)]
    pub features: Vec<String>,

    #[arg(short = 'p', long)]
    pub pkg: String,
}

#[derive(Debug, Clone, Parser)]
pub struct RemoveCommand {
    pub crate_name: String,

    #[arg(short = 'p', long)]
    pub pkg: String,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            Commands::Add(add) => {
                if add.crate_name.contains("@") {
                    let (name, ver) = add
                        .crate_name
                        .split("@")
                        .map(|v| v.to_string())
                        .collect_tuple()
                        .unwrap();

                    add_crate(name, Some(ver), add.features, add.pkg).await?;
                } else {
                    add_crate(add.crate_name, None, add.features, add.pkg).await?;
                }
            }

            Commands::Remove(rm) => {
                remove_crate(rm.crate_name, rm.pkg)?;
            }
        }

        Ok(())
    }
}

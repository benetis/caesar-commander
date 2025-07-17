use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "caesar-commander")]
pub struct Cli {
    /// Initial path for the left pane (defaults to home directory)
    #[arg(long)]
    left: Option<PathBuf>,

    /// Initial path for the right pane (defaults to same as left or home)
    #[arg(long)]
    right: Option<PathBuf>,
}

impl Cli {
    pub fn parse_and_paths() -> (PathBuf, PathBuf) {
        let cli = Cli::parse();
        let home = dirs::home_dir().expect("Could not find home directory");

        let left = cli.left.clone().unwrap_or_else(|| home.clone());

        let right = cli
            .right
            .unwrap_or_else(|| cli.left.unwrap_or(home.clone()));

        (left, right)
    }
}
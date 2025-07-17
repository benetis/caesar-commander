use clap::Parser;
use std::path::PathBuf;
use crate::model::params::Params;

#[derive(Parser, Debug)]
#[command(name = "caesar-commander")]
pub struct Cli {
    /// Initial path for the left pane (defaults to home directory)
    #[arg(long)]
    left: Option<PathBuf>,

    /// Initial path for the right pane (defaults to same as left or home)
    #[arg(long)]
    right: Option<PathBuf>,

    /// UI scale factor: how many physical pixels per logical point
    #[arg(long, default_value_t = 1.0)]
    pub scale: f32,
}

impl Cli {
    pub fn new() -> Params {
        let cli = Cli::parse();
        let home = dirs::home_dir().expect("Could not find home directory");

        let left = cli.left.clone().unwrap_or_else(|| home.clone());

        let right = cli
            .right
            .unwrap_or_else(|| cli.left.unwrap_or(home.clone()));

        Params {
            left_path: left,
            right_path: right,
            scale: cli.scale,
        }
    }
}
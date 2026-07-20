use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gitui-view", version, about = "A terminal-based Git repository browser with inline and side-by-side diff views")]
pub struct CliArgs {
    #[arg(
        short = 'd',
        long = "directory",
        default_value = ".",
        env = "GIT_DIR",
        help = "Set the git directory"
    )]
    pub repo_path: PathBuf,

    #[arg(
        short = 't',
        long = "theme",
        help = "Path to a custom theme file"
    )]
    pub theme: Option<PathBuf>,
}

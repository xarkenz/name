use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub input_filename: std::path::PathBuf,

    #[arg(short, long, help = "Enable debug mode")]
    pub debug: bool,
}

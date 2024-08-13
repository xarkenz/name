use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    pub(crate) input_filename: std::path::PathBuf,
    
    #[arg(short, long, help="Enable debug mode")]
    pub(crate) debug: bool,
}
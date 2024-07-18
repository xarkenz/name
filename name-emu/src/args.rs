use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    pub(crate) input_filename: std::path::PathBuf,
}
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    pub(crate) input_filename: std::path::PathBuf,
    pub(crate) output_filename: std::path::PathBuf,

    #[arg(short, long)]
    pub(crate) verbose: bool,
}
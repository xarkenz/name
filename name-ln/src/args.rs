use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    
    #[arg(short, long)]
    pub(crate) input_filenames: std::path::PathBuf, //Vec<std::path::PathBuf>,
    
    #[arg(short, long)]
    pub(crate) output_filename: std::path::PathBuf,
}
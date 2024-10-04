use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub input_filenames: std::path::PathBuf, //Vec<std::path::PathBuf>,

    #[arg(short, long)]
    pub output_filename: std::path::PathBuf,
}


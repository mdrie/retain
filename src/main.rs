use clap::Parser;

// Idenfify backup copies to be deleted according to a given retention policy.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long, default_value = "")]
    regex: String,
    /// The path to the file to read
    #[arg(short, long, default_value = ".")]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}

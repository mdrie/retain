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

fn retain<'a>(input: &Vec<&'a str>) -> Vec<&'a str> {
    Vec::clone(input)
}

#[test]
fn keep_first_and_last() {
    let result = retain(&vec!["a", "b", "c"]);
    assert_eq!(*(result.first().unwrap_or(&"")), "a");
    assert_eq!(*(result.last().unwrap_or(&"")), "c");
}

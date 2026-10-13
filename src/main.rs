use std::fs;
use std::path::Path;
use clap::Parser;
use md_frontmatter_converter_rs::{finder, formatter};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory for input markdown files.
    #[arg(long)]
    dir: String,

    /// Output directory
    #[arg(long)]
    output: String
}

fn main() {
    let args = Args::parse();
    let output_dir = Path::new(&args.output);

    let paths = match finder::find_markdown_files(&args.dir) {
        Ok(paths) => {
            println!("Found {} markdown files.", paths.len());
            paths
        },
        Err(error) => {
            eprintln!("An error occured: {}", error);
            return
        }
    };

    for path in paths {
        println!("Processing {} ...", &path.display().to_string());
        let content = formatter::read_markdown_file(&path);
        let (frontmatter, markdown) = match formatter::get_yaml_front_matter(&content) {
            Some(result) => result,
            None => {
                eprintln!("An error occured when parsing.");
                return
            }
        };
        let save_path = output_dir.join(path.file_name().unwrap());

    }
}

use clap::Parser;

use md_frontmatter_converter_rs::formatter;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Directory for input markdown files.
    #[arg(long)]
    dir: String,
}


fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

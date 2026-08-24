use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory for input markdown files.
    #[arg(long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    // placeholder
    println!("{}", &args.dir);
}

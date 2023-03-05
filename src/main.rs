use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file / directory
    #[arg(required = true)]
    input: String,

    /// Output file / directory
    #[arg(required = true)]
    output: String,

    /// File format (jpg, vtf, etc.)
    #[arg(short, long)]
    format: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Convert the file
    match rvtf::convert(args.input, args.output, &args.format) {
        Err(e) => eprintln!("{}", e),
        Ok(()) => println!("Successfully converted file(s)"),
    };
}

use clap::Parser;

#[derive(Parser)]
#[command(name = "fl")]
#[command(author = "Shuyu Liang", version = "0.1", about = "A log formatter CLI", long_about = None)]
struct Args {
    // log content
    #[arg(short, long)]
    content: String,
}

fn main() {
    let args = Args::parse();
}

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Neha Barde",
    about = "A Sentiment Analysis tool"
)]
struct Cli {
    #[clap(subcommand)]
    //command: Option<Commands>,
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Neha Barde")]
    /*infer {
        #[clap(short, long)]
        input: String,
    },*/
    infer,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::infer) => {
            onnx_pytorch::analyze_sent();
        }
        None => println!("No subcommand was used"),
    }
}

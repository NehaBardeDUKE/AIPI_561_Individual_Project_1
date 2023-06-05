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
    Infer {
        #[clap(short, long)]
        input: String,
    },
    //infer,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Infer { input }) => {
            sentiment_analyze::sentiment_out(&input);
        }
        None => println!("No subcommand was used"),
    }
}

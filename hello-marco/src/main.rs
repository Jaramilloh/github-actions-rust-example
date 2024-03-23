//A command-line tool that plays Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
// hello::marco_polo(&name)
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command was used"),
    }
}

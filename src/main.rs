use clap::{Parser, Subcommand};
use hyper::Uri;

#[derive(Parser)]
struct Cli {
    /// Base URL of the API server
    url: hyper::Uri,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all todos
    List,
    /// Create a new todo
    Create {
        /// The todo body
        body: String,
    },
    /// Update a todo
    Update {
        /// The todo ID
        id: String,
        /// The todo body
        body: String,
        /// Mark todo as completed
        #[arg(short, long)]
        completed: bool,
    },
    /// Delete a todo
    Delete {
        /// The todo ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut uri_builder = Uri::builder();
    if let Some(scheme) = cli.url.scheme() {
        uri_builder = uri_builder.scheme(scheme.clone());
    }

    if let Some(authority) = cli.url.authority() {
        uri_builder = uri_builder.authority(authority.clone());
    }

    match cli.command {
        Commands::List => {}
        Commands::Create { body } => {}
        Commands::Update {
            id,
            body,
            completed,
        } => {}
        Commands::Delete { id } => {}
    }

    Ok(())
}

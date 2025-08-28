use clap::{Parser, Subcommand};
use reqwest::{Method, Url};
use serde_json::json;

mod request;

#[derive(Parser)]
struct Cli {
    /// Base URL of the API server
    url: Url,

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

    let mut url = cli.url.clone();

    match cli.command {
        Commands::List => {
            url.set_path("v1/todos");
            // Await the request and handle any error, converting it to a boxed error with Send + Sync
            request::request(url, Method::GET, None).await?;
        }
        Commands::Create { body } => {
            url.set_path("v1/todos");
            let body = json!({"body": body});
            request::request(url, Method::POST, Some(body.to_string())).await?;
        }
        Commands::Update {
            id,
            body,
            completed,
        } => {}
        Commands::Delete { id } => {}
    }

    Ok(())
}

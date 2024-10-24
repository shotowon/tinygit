use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Query {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    CatFile {
        // Pretty print of the contents.
        #[arg(short, long)]
        pretty: bool,

        // object-sha
        #[arg(required = true)]
        sha: String,
    },
    HashObject {
        #[arg(short, long)]
        write: bool,

        #[arg(required = true)]
        filepath: String,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,

        #[arg(required = true)]
        tree_sha: String,
    },
    Init,
}

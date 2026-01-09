use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Switch to session
    #[command(alias = "s")]
    Session {
        session: String,
    },
    /// Add a new task
    #[command(alias = "a")]
    Add {
        #[arg(required = true)]
        text: String,
    },
    /// Edit the task with index
    #[command(alias = "e")]
    Edit {
        #[arg(required = true)]
        index: usize,
        #[arg(required = true)]
        text: String,
    },
    /// Mark task as done
    #[command(alias = "d")]
    Done {
        #[arg(required = true)]
        index: usize,
    },
    /// Mark task as undone
    #[command(alias = "u")]
    Undone {
        #[arg(required = true)]
        index: usize,
    },
    /// Remove a task
    #[command(alias = "r")]
    Rm {
        #[arg(required = true)]
        index: usize,
    },
    /// List all sessions
    #[command(alias = "ss")]
    Sessions,
    /// List tasks (default if no args)
    #[command(alias = "l")]
    Ls,
}

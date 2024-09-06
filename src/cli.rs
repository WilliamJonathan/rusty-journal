use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Grava tarefas no arquivo de diário.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove uma entrada do arquivo de diário por posição.
    Done {
        #[structopt()]
        position: usize,
    },
    /// Lista todas as tarefas no arquivo de diário.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use um arquivo de diário diferente.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
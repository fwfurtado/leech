use clap::{CommandFactory, Parser};
use clap_complete::generate;
use leech::cli::{Cli, Commands, CompletionArgs, SyncArgs};
use leech::use_case::backup_repositories_for;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Backup(
            SyncArgs { organization, limit }
        ) => backup_repositories_for(organization, limit).await,

        Commands::Completion(
            CompletionArgs {
                shell
            }
        ) => {
            let mut cli = Cli::command();

            let name = cli.get_name().to_string();

            generate(shell, &mut cli, name.clone(), &mut std::io::stdout());
        }
    }
}

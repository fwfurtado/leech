use std::io::Write;
use clap::builder::PossibleValue;
use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell};
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;

#[derive(Parser, Debug)]
#[command(name = "leech")]
#[command(version, about="Manage git repositories backup", long_about = None)]
#[command(name = "completion-derive")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Backup all repositories from GitHub based on limit")]
    Backup(SyncArgs),

    #[command(about = "Generate shell completion")]
    Completion(CompletionArgs),
}

#[derive(Parser, Debug)]
pub struct SyncArgs {
    #[arg(short, long, required = true)]
    pub organization: String,

    #[arg(short, long, default_value_t = 10)]
    pub limit: u16,
}

#[derive(Parser, Debug)]
pub struct CompletionArgs {
    #[arg(value_enum)]
    pub shell: AvailableShells,
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AvailableShells {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
    NuShell,
    Fig,
}

impl ValueEnum for AvailableShells {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            AvailableShells::Bash,
            AvailableShells::Elvish,
            AvailableShells::Fish,
            AvailableShells::PowerShell,
            AvailableShells::Zsh,
            AvailableShells::NuShell,
            AvailableShells::Fig,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AvailableShells::Bash => PossibleValue::new("bash"),
            AvailableShells::Elvish => PossibleValue::new("elvish"),
            AvailableShells::Fish => PossibleValue::new("fish"),
            AvailableShells::PowerShell => PossibleValue::new("powershell"),
            AvailableShells::Zsh => PossibleValue::new("zsh"),
            AvailableShells::NuShell => PossibleValue::new("nushell"),
            AvailableShells::Fig => PossibleValue::new("fig"),
        })
    }
}


impl Generator for AvailableShells {
    fn file_name(&self, name: &str) -> String {
        match self {
            AvailableShells::Bash => Shell::Bash.file_name(name),
            AvailableShells::Elvish => Shell::Elvish.file_name(name),
            AvailableShells::Fish => Shell::Fish.file_name(name),
            AvailableShells::PowerShell => Shell::PowerShell.file_name(name),
            AvailableShells::Zsh => Shell::Zsh.file_name(name),
            AvailableShells::NuShell => Nushell.file_name(name),
            AvailableShells::Fig => Fig.file_name(name),
        }
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        match self {
            AvailableShells::Bash => Shell::Bash.generate(cmd, buf),
            AvailableShells::Elvish => Shell::Elvish.generate(cmd, buf),
            AvailableShells::Fish => Shell::Fish.generate(cmd, buf),
            AvailableShells::PowerShell => Shell::PowerShell.generate(cmd, buf),
            AvailableShells::Zsh => Shell::Zsh.generate(cmd, buf),
            AvailableShells::NuShell => Nushell.generate(cmd, buf),
            AvailableShells::Fig => Fig.generate(cmd, buf),
        }
    }
}
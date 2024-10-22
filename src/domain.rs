use std::fmt::Display;
use std::path::Path;
use std::process::Output;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug)]
pub enum Error {
    Clone(String, std::io::Error),
    Pull(String, std::io::Error),
}


impl Error {
    fn clone_for(name: String) -> impl Fn(std::io::Error) -> Self {
        move |e| Error::Clone(name.clone(), e)
    }

    fn pull_for(name: String) -> impl Fn(std::io::Error) -> Self {
        move |e| Error::Pull(name.clone(), e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Clone(name, e) => write!(f, "Error cloning repository {}: {}", name, e),
            Error::Pull(name, e) => write!(f, "Error pulling repository {}: {}", name, e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    name: String,

    #[serde(alias = "nameWithOwner")]
    owner_name: String,
}

impl Repository {

    pub fn dir_name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn sync_to_local(&self) -> Result<Output, Error> {

        let path = Path::new(self.name.as_str());

        if path.is_dir() {
            Command::new("git")
                .current_dir(path)
                .args(&["pull"])
                .output()
                .map_err(Error::pull_for(self.name.clone()))
                .await
        } else {
            Command::new("gh")
                .args(&["repo", "clone", self.owner_name.as_str()])
                .output()
                .map_err(Error::clone_for(self.name.clone()))
                .await
        }
    }
}
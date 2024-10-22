use std::fmt::Display;
use crate::domain::Repository;
use futures::TryFutureExt;
use std::io;
use tokio::process::Command;


#[derive(Debug)]
pub enum Error {
    ListError(String, io::Error),
    SerdeError(String, serde_json::Error),
}

impl Error {
    fn list_for(name: String) -> impl Fn(io::Error) -> Self {
        move |e| Error::ListError(name.clone(), e)
    }

    fn serde_for(name: String) -> impl Fn(serde_json::Error) -> Self {
        move |e| Error::SerdeError(name.clone(), e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ListError(name, e) => write!(f, "Error fetching repositories for {}: {}", name, e),
            Error::SerdeError(name, e) => write!(f, "Error deserializing repositories for {}: {}", name, e),
        }
    }
}

pub async fn get_repositories(organization: &str, limit: u16) -> Result<Vec<Repository>, Error> {
    let json_result = Command::new("gh")
        .args(&[
            "repo",
            "list",
            "--limit", limit.to_string().as_str(),
            "--json", "name,nameWithOwner",
            organization,
        ])
        .output()
        .map_err(Error::list_for(organization.to_string()))
        .await?;

    let repos = serde_json::from_slice(&json_result.stdout).map_err(Error::serde_for(organization.to_string()))?;

    Ok(repos)
}
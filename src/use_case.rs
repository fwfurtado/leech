use std::process::Output;
use futures::{StreamExt, TryStreamExt};
use indicatif::ProgressStyle;
use crate::domain::{Error, Repository};
use crate::gateway::get_repositories;


pub async fn backup_repositories_for(org: String, limit: u16) {

    let repos: Vec<Repository> = match get_repositories(org.as_str(), limit).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error fetching repositories: {:?}", e);
            return;
        }
    };

    let bar = indicatif::ProgressBar::new(repos.len() as u64);

    let style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").expect("failed to set style");

    bar.set_style(style);

    let result = futures::stream::iter(repos.into_iter())
        .map(|repo: Repository| tokio::task::spawn({
            let value = bar.clone();
            async move { download_unit(&repo, &value).await }
        }))
        .buffered(num_cpus::get())
        .try_collect::<Vec<Result<Output, Error>>>()
        .await;


    bar.finish();


    match result {
        Ok(res) => report(res),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

async fn download_unit(repository: &Repository, bar: &indicatif::ProgressBar) -> Result<Output, Error> {
    bar.set_message(format!("Processing: {}", repository.dir_name()));

    let result = repository.sync_to_local().await;

    bar.inc(1);

    result
}

fn report(result: Vec<Result<Output, Error>>) {
    for r in result.iter().filter(|r| r.is_err()).map(|r| r.as_ref().unwrap_err()) {
        eprintln!("Error: {:?}", r);
    }
}
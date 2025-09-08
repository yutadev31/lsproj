use std::{
    fs::read_dir,
    io::{Write, stdout},
    path::PathBuf,
};

use clap::Parser;
use crossterm::{
    queue,
    style::{Color, Print, SetForegroundColor},
};
use git2::Repository;

#[derive(Parser)]
struct Cli {
    #[arg(default_value_t = String::from("."))]
    pub path: String,

    #[arg(short, long, default_value_t = false)]
    pub remotes: bool,

    #[arg(short, long, default_value_t = false)]
    pub git_only: bool,
}

enum RemoteType {
    Github,
    Other,
}

struct ProjectInfo {
    git: bool,
    remote_url: Option<String>,
    remote_type: Option<RemoteType>,
}

fn get_remote_url(repo: &Repository) -> Option<String> {
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|url| url.to_string())
}

fn url_to_remote_type(url: String) -> RemoteType {
    if url.contains("github.com") {
        RemoteType::Github
    } else {
        RemoteType::Other
    }
}

fn open_repository(path: PathBuf) -> ProjectInfo {
    match Repository::open(path) {
        Ok(repo) => {
            let url = get_remote_url(&repo);
            ProjectInfo {
                git: true,
                remote_url: url.clone(),
                remote_type: url.map(url_to_remote_type),
            }
        }
        Err(_) => ProjectInfo {
            git: false,
            remote_url: None,
            remote_type: None,
        },
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut directories: Vec<(String, ProjectInfo)> = read_dir(cli.path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                let repo_name = entry.file_name().to_string_lossy().to_string();
                let project_info = open_repository(path);

                if cli.git_only && !project_info.git {
                    None
                } else {
                    Some((repo_name, project_info))
                }
            } else {
                None
            }
        })
        .collect();

    directories.sort_by_key(|pair| pair.0.clone());

    let name_width = directories
        .iter()
        .map(|pair| pair.0.len() + 2)
        .max()
        .unwrap_or(0);

    for (name, repo) in directories {
        let name_color = if repo.git { Color::Red } else { Color::Blue };
        let type_icon = if repo.git { "󰊢 " } else { "󰉋 " };

        let name_text = format!("{:<width$}", name, width = name_width);

        queue!(
            stdout(),
            SetForegroundColor(name_color),
            Print(type_icon),
            Print(name_text),
            SetForegroundColor(Color::Reset),
        )?;

        if cli.remotes && repo.remote_type.is_some() && repo.remote_url.is_some() {
            let remote_type = repo.remote_type.unwrap();

            let remote_icon = match remote_type {
                RemoteType::Github => "󰊤 ",
                RemoteType::Other => "󰊢 ",
            };

            queue!(
                stdout(),
                Print(remote_icon),
                SetForegroundColor(Color::Blue),
                Print(repo.remote_url.unwrap()),
                SetForegroundColor(Color::Reset),
            )?;
        }

        queue!(stdout(), Print("\n"))?;
    }
    stdout().flush()?;

    Ok(())
}

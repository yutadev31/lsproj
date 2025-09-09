pub mod project;

use std::{
    fs::read_dir,
    io::{Write, stdout},
};

use clap::Parser;
use crossterm::{
    queue,
    style::{Color, Print, SetForegroundColor},
};

use crate::project::Project;

#[derive(Parser)]
pub struct Cli {
    #[arg(default_value_t = String::from("."))]
    pub path: String,

    #[arg(short, long, default_value_t = false)]
    pub remote: bool,

    #[arg(short, long, default_value_t = false)]
    pub git_only: bool,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut directories: Vec<(String, Project)> = read_dir(&self.path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    let repo_name = entry.file_name().to_string_lossy().to_string();
                    Some((repo_name, Project::open(&path)?))
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

        for (name, project) in directories {
            let (name_color, type_icon) = if project.repo.is_some() {
                (Color::Red, "󰊢 ")
            } else {
                (Color::Blue, "󰉋 ")
            };

            let name_text = format!("{:<width$}", name, width = name_width);

            queue!(
                stdout(),
                SetForegroundColor(name_color),
                Print(type_icon),
                Print(name_text),
                SetForegroundColor(Color::Reset),
            )?;

            if let Some(repo) = project.repo {
                if let Some(remote) = repo.remote
                    && self.remote
                {
                    queue!(
                        stdout(),
                        SetForegroundColor(Color::Blue),
                        Print("󰊢 "),
                        Print(remote),
                        SetForegroundColor(Color::Reset),
                    )?;
                }
            }

            queue!(stdout(), Print("\n"))?;
        }
        stdout().flush()?;

        Ok(())
    }
}

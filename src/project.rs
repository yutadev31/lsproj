use std::path::Path;

use git2::Repository as GitRepository;

pub struct Project {
    pub repo: Option<Repository>,
}

impl Project {
    pub fn open(path: &Path) -> Option<Self> {
        Some(Self {
            repo: Repository::open(path),
        })
    }
}

pub struct Repository {
    pub remote: Option<String>,
}

impl Repository {
    pub fn open(path: &Path) -> Option<Self> {
        let repo = GitRepository::open(path).ok()?;

        let remote = repo.find_remote("origin").ok()?;

        Some(Self {
            remote: remote.url().map(|url| url.to_string()),
        })
    }
}

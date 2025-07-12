use crate::errors::Result;
use git2::{Commit, Oid, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub message: String,
    pub timestamp: i64,
    pub repository_url: String,
    pub branch: String,
    pub files_changed: Vec<String>,
}

pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(GitRepository { repo })
    }

    pub fn discover<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)?;
        Ok(GitRepository { repo })
    }

    pub fn get_commit_info(&self, commit_hash: &str) -> Result<CommitInfo> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;

        let repository_url = self
            .get_remote_url()
            .unwrap_or_else(|| "unknown".to_string());
        let branch = self
            .get_current_branch()
            .unwrap_or_else(|| "unknown".to_string());
        let files_changed = self.get_changed_files(&commit)?;

        let author = commit.author();
        let committer = commit.committer();

        Ok(CommitInfo {
            hash: commit.id().to_string(),
            short_hash: format!("{:.7}", commit.id().to_string()),
            author_name: author.name().unwrap_or("unknown").to_string(),
            author_email: author.email().unwrap_or("unknown").to_string(),
            committer_name: committer.name().unwrap_or("unknown").to_string(),
            committer_email: committer.email().unwrap_or("unknown").to_string(),
            message: commit.message().unwrap_or("").to_string(),
            timestamp: commit.time().seconds(),
            repository_url,
            branch,
            files_changed,
        })
    }

    pub fn get_head_commit_info(&self) -> Result<CommitInfo> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        self.get_commit_info(&commit.id().to_string())
    }

    fn get_remote_url(&self) -> Option<String> {
        self.repo
            .find_remote("origin")
            .ok()
            .and_then(|remote| remote.url().map(|url| url.to_string()))
    }

    fn get_current_branch(&self) -> Option<String> {
        self.repo
            .head()
            .ok()
            .and_then(|head| head.shorthand().map(|name| name.to_string()))
    }

    fn get_changed_files(&self, commit: &Commit) -> Result<Vec<String>> {
        let mut files = Vec::new();

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_options = git2::DiffOptions::new();
        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_options),
        )?;

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }
}

pub fn get_git_info_from_env() -> Result<CommitInfo> {
    use std::env;

    // Check if we're in GitHub Actions
    if env::var("GITHUB_ACTIONS").is_ok() {
        return get_git_info_from_github_actions();
    }

    // Try to get commit hash from environment (set by git hooks)
    let commit_hash = env::var("GIT_COMMIT")
        .or_else(|_| env::var("GITHUB_SHA"))
        .or_else(|_| -> std::result::Result<String, std::env::VarError> {
            // Try to get from current directory
            let repo = GitRepository::discover(".").map_err(|_| std::env::VarError::NotPresent)?;
            let commit_info = repo
                .get_head_commit_info()
                .map_err(|_| std::env::VarError::NotPresent)?;
            Ok(commit_info.hash)
        })
        .map_err(|_| {
            crate::errors::GitFriendsError::Unknown("Could not determine commit hash".to_string())
        })?;

    let repo = GitRepository::discover(".")?;
    repo.get_commit_info(&commit_hash)
}

pub fn get_git_info_from_github_actions() -> Result<CommitInfo> {
    use std::env;

    // GitHub Actions environment variables
    let commit_hash = env::var("GITHUB_SHA")
        .map_err(|_| crate::errors::GitFriendsError::Unknown("GITHUB_SHA not found".to_string()))?;

    let repository_url = env::var("GITHUB_SERVER_URL")
        .and_then(|server| env::var("GITHUB_REPOSITORY").map(|repo| format!("{}/{}", server, repo)))
        .unwrap_or_else(|_| {
            env::var("GITHUB_REPOSITORY")
                .map(|repo| format!("https://github.com/{}", repo))
                .unwrap_or_else(|_| "unknown".to_string())
        });

    let branch = env::var("GITHUB_REF_NAME")
        .or_else(|_| env::var("GITHUB_HEAD_REF"))
        .or_else(|_| env::var("GITHUB_BASE_REF"))
        .unwrap_or_else(|_| "unknown".to_string());

    // Try to get commit info from git if available, otherwise construct from env
    if let Ok(repo) = GitRepository::discover(".") {
        let mut commit_info = repo.get_commit_info(&commit_hash)?;

        // Override with GitHub Actions specific info
        commit_info.repository_url = repository_url;
        commit_info.branch = branch;

        Ok(commit_info)
    } else {
        // Construct commit info from environment variables
        let author_name = env::var("GITHUB_ACTOR").unwrap_or_else(|_| "unknown".to_string());
        let author_email = format!("{}@users.noreply.github.com", author_name);

        // Get commit message from event payload if available
        let message = get_commit_message_from_github_event()
            .unwrap_or_else(|_| "GitHub Actions commit".to_string());

        Ok(CommitInfo {
            hash: commit_hash.clone(),
            short_hash: format!("{:.7}", commit_hash),
            author_name: author_name.clone(),
            author_email: author_email.clone(),
            committer_name: author_name,
            committer_email: author_email,
            message,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            repository_url,
            branch,
            files_changed: vec![], // Could be populated from GitHub API if needed
        })
    }
}

fn get_commit_message_from_github_event() -> Result<String> {
    use std::env;
    use std::fs;

    // Try to read from GitHub event payload
    if let Ok(event_path) = env::var("GITHUB_EVENT_PATH") {
        if let Ok(event_data) = fs::read_to_string(event_path) {
            if let Ok(event_json) = serde_json::from_str::<serde_json::Value>(&event_data) {
                // Try different event types
                if let Some(head_commit) = event_json.get("head_commit") {
                    if let Some(message) = head_commit.get("message") {
                        if let Some(msg_str) = message.as_str() {
                            return Ok(msg_str.to_string());
                        }
                    }
                }

                // Try pull request
                if let Some(pull_request) = event_json.get("pull_request") {
                    if let Some(title) = pull_request.get("title") {
                        if let Some(title_str) = title.as_str() {
                            return Ok(title_str.to_string());
                        }
                    }
                }

                // Try commits array
                if let Some(commits) = event_json.get("commits") {
                    if let Some(commits_array) = commits.as_array() {
                        if let Some(first_commit) = commits_array.first() {
                            if let Some(message) = first_commit.get("message") {
                                if let Some(msg_str) = message.as_str() {
                                    return Ok(msg_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(crate::errors::GitFriendsError::Unknown(
        "Could not extract commit message from GitHub event".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_repository_creation() {
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        // This would normally fail because there are no commits
        // But we can test the basic functionality
        assert!(GitRepository::open(temp_dir.path()).is_ok());
    }
}

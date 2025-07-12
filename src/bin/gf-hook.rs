use clap::{Command, Arg, ArgAction};
use git_friends::{git::get_git_info_from_env, Result};
use log::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let matches = Command::new("gf-hook")
        .version("0.1.0")
        .about("Git Friends commit hook - sends commit information to gf-server")
        .arg(
            Arg::new("server-url")
                .short('s')
                .long("server-url")
                .value_name("URL")
                .help("The URL of the gf-server")
                .default_value("http://localhost:8080")
        )
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .value_name("TOKEN")
                .help("Authentication token")
        )
        .arg(
            Arg::new("commit")
                .short('c')
                .long("commit")
                .value_name("COMMIT_HASH")
                .help("Specific commit hash to process (defaults to HEAD)")
        )
        .arg(
            Arg::new("github-actions")
                .long("github-actions")
                .help("Force GitHub Actions mode (auto-detected by default)")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Don't actually send the request, just show what would be sent")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    
    let server_url = matches.get_one::<String>("server-url").unwrap();
    let token = matches.get_one::<String>("token");
    let commit_hash = matches.get_one::<String>("commit");
    let dry_run = matches.get_flag("dry-run");
    let force_github_actions = matches.get_flag("github-actions");
    
    // Check for token in environment if not provided
    let token_env = std::env::var("GIT_FRIENDS_TOKEN").ok();
    let token = token.map(|s| s.as_str()).or_else(|| token_env.as_deref());
    
    info!("Starting gf-hook");
    
    // Check if we're in GitHub Actions or forced
    let is_github_actions = force_github_actions || std::env::var("GITHUB_ACTIONS").is_ok();
    
    if is_github_actions {
        info!("Running in GitHub Actions mode");
    }
    
    // Get commit information
    let commit_info = if let Some(hash) = commit_hash {
        // Use specific commit hash
        let repo = git_friends::git::GitRepository::discover(".")?;
        repo.get_commit_info(hash)?
    } else if force_github_actions {
        // Force GitHub Actions mode
        git_friends::git::get_git_info_from_github_actions()?
    } else {
        // Use environment or HEAD (auto-detects GitHub Actions)
        get_git_info_from_env()?
    };
    
    info!("Commit info: {} by {} - {}", 
          commit_info.short_hash, 
          commit_info.author_name, 
          commit_info.message.lines().next().unwrap_or(""));
    
    if dry_run {
        println!("DRY RUN - Would send to {}", server_url);
        println!("Commit info: {}", serde_json::to_string_pretty(&commit_info)?);
        return Ok(());
    }
    
    // Send to server
    let client = reqwest::Client::new();
    let mut request_builder = client.post(&format!("{}/webhook", server_url))
        .json(&commit_info);
    
    if let Some(token) = token {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
    }
    
    match request_builder.send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Successfully sent commit information to server");
            } else {
                error!("Server returned error: {}", response.status());
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to send commit information: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

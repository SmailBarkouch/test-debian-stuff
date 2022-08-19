use std::env;
use github_release::{AuthenticatedUser, RepoInfo};

fn main() {
    let auth_user = AuthenticatedUser::new(None).unwrap();
    let repo_info = RepoInfo {
        owner: "smailbarkouch".to_string(),
        repo_name: "test-debian-stuff".to_string()
    };

    auth_user.get_latest_release(&repo_info).unwrap();

    println!("Success!");
}

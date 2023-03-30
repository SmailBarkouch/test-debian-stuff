use std::env;
use std::fs::File;
use std::process::Command;
use gh_release::{ReleaseClient, RepoInfo};
use gh_release::release::{CreateReleaseInfo, ReleaseInfo, TagInfo};

fn main() {
    let auth_user = ReleaseClient::new("b4f3729ff174de5f2057052f8e8ad53a9575de4c".to_string()).unwrap();
    let repo_info = RepoInfo {
        owner: "smailbarkouch",
        repo_name: "test-debian-stuff"
    };

    let hash_bytes = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap()
        .stdout;

    let hash = String::from_utf8_lossy(hash_bytes.as_slice())
        .trim()
        .to_string();

    let tag_info = TagInfo {
        tag: "0.0.7".to_string(),
        message: "The first release ever.".to_string(),
        object: hash,
        type_tagged: "commit".to_string()
    };

    auth_user.create_a_tag(&repo_info, &tag_info).unwrap();
    
    let release_info = CreateReleaseInfo {
        tag_name: "0.0.7".to_string(),
        target_commitish: None,
        name: Some("This is the new version".to_string()),
        body: None,
        draft: None,
        prerelease: None,
        discussion_category_name: None,
        generate_release_notes: Some(true),
        make_latest: None
    };



    auth_user.create_a_release(&repo_info, &release_info).unwrap();
}

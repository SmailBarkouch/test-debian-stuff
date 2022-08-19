use std::env;
use std::fs::File;
use github_release::{AuthenticatedUser, RepoInfo};

fn main() {
    let auth_user = AuthenticatedUser::new(None).unwrap();
    let repo_info = RepoInfo {
        owner: "smailbarkouch".to_string(),
        repo_name: "test-debian-stuff".to_string()
    };

    println!("LATEST FIELD: {}", auth_user.get_release_by_tag_name(&repo_info, "0.1").unwrap().id);
    
//     74857070
    
    // let info = auth_user.upload_release_asset(&repo_info, 74857070, "android.zip", "application/zip", File::open("/home/smailbarkouch/Downloads/android.zip").unwrap(), None).unwrap();

    // println!("THE INFO: {:?}", info);

}

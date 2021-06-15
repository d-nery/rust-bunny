extern crate percent_encoding;

use percent_encoding::utf8_percent_encode;

use super::{Command, FRAGMENT};

pub struct GitHub;

impl Command for GitHub {
    fn get_name(&self) -> String {
        "gh".to_string()
    }

    fn get_description(&self) -> String {
        "Usage: gh [user] [repo] -> Go to github, a specific user/organization or repository"
            .to_string()
    }

    fn construct_url(&self, args: Option<Vec<&str>>) -> String {
        if args == None {
            return "https://github.com".to_string();
        }

        let combined_args = args.unwrap().join("/");
        let encoded_query = utf8_percent_encode(&combined_args, FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);

        github_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_profile_url_with_gh() {
        assert_eq!(GitHub.construct_url(None), "https://github.com");
    }

    #[test]
    fn test_construct_profile_url_with_repo_url() {
        assert_eq!(
            GitHub.construct_url(Some(vec!["facebook"])),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_construct_search_url_with_repo_url() {
        assert_eq!(
            GitHub.construct_url(Some(vec!["facebook", "docusaurus"])),
            "https://github.com/facebook/docusaurus"
        );
    }
}

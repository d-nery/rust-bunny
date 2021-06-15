extern crate percent_encoding;
use percent_encoding::utf8_percent_encode;

use super::{Command, FRAGMENT};

pub struct Twitter;

impl Command for Twitter {
    fn get_name(&self) -> String {
        "tw".to_string()
    }

    fn get_description(&self) -> String {
        "Usage: tw [@profile|search] -> Go to twitter, a specific profile or search".to_string()
    }

    fn construct_url(&self, args: Option<Vec<&str>>) -> String {
        if args == None {
            return "https://twitter.com".to_string();
        }

        let query = args.unwrap().join(" ");

        if query.starts_with("@") {
            construct_profile_url(&query[1..])
        } else {
            construct_search_url(&query)
        }
    }
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let search_url = format!("https://twitter.com/search?q={}", encoded_query);

    search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_url() {
        assert_eq!(Twitter.construct_url(None), "https://twitter.com");
    }

    #[test]
    fn test_construct_url_query() {
        assert_eq!(
            Twitter.construct_url(Some(vec!["hello", "world"])),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_url_profile() {
        assert_eq!(
            Twitter.construct_url(Some(vec!["@user"])),
            "https://twitter.com/user"
        );
    }

    #[test]
    fn test_construct_profile_url() {
        let fake_profile = "user";
        assert_eq!(
            construct_profile_url(fake_profile),
            "https://twitter.com/user"
        );
    }

    #[test]
    fn test_construct_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_search_url(fake_query),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}

extern crate percent_encoding;

use super::Command;

pub struct List;

impl Command for List {
    fn get_name(&self) -> String {
        "list".to_string()
    }

    fn get_description(&self) -> String {
        "List available commands".to_string()
    }

    fn construct_url(&self, _args: Option<Vec<&str>>) -> String {
        uri!("/list").to_string()
    }
}

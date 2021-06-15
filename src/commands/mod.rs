use percent_encoding::{AsciiSet, CONTROLS};

pub mod github;
pub mod google;
pub mod list;
pub mod twitter;

pub const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub trait Command {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn construct_url(&self, args: Option<Vec<&str>>) -> String;
}

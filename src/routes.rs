use rocket::response::Redirect;

use crate::commands::*;
use crate::parser;

pub const COMMANDS: [&dyn Command; 3] = [&github::GitHub, &twitter::Twitter, &list::List];

#[get("/parse?<q>")]
pub async fn parse(q: String) -> Redirect {
    let (cmd, args) = parser::get_command_and_args_from_query_string(&q);

    for command in COMMANDS.iter() {
        if cmd == command.get_name() {
            return Redirect::to(command.construct_url(args));
        }
    }

    Redirect::to(google::construct_search_url(&q))
}

#[get("/list")]
pub async fn list_all() -> String {
    let mut resp = Vec::<String>::new();

    for command in COMMANDS.iter() {
        resp.push(format!(
            "{} -> {}",
            command.get_name(),
            command.get_description()
        ));
    }

    resp.join("\n")
}

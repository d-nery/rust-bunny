use rocket::response::Redirect;

use crate::commands::*;
use crate::parser;

#[get("/parse?<q>")]
pub async fn parse(q: String) -> Redirect {
    let cmd = parser::get_command_from_query_string(&q);

    let redirect_url = match cmd {
        "tw" => twitter::construct_url(&q),
        "gh" => github::construct_url(&q),
        _ => google::construct_search_url(&q),
    };

    Redirect::to(redirect_url)
}

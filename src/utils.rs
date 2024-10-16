use chrono::NaiveDateTime;
use dotenv::dotenv;
use regex::Regex;
use std::env;

pub fn slug_gen(slug_in: &str) -> String {
    let lowercase = slug_in.to_lowercase();

    let re = Regex::new(r"[^a-z0-9\s]").unwrap();
    let alphanumeric = re.replace_all(&lowercase, "");

    let hyphenated = alphanumeric.replace(" ", "-");

    let re = Regex::new(r"-+").unwrap();
    let single_hyphen = re.replace_all(&hyphenated, "-");

    single_hyphen
        .trim_matches('-')
        .to_string()
}

pub fn get_current_timestamp() -> NaiveDateTime {
    chrono::Local::now().naive_utc()
}

pub fn get_site_identifier() -> String {
    dotenv().ok();
    env::var("SITE_IDENTIFIER").expect("SITE_IDENTIFIER must be set")
}

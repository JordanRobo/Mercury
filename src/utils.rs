use chrono::NaiveDateTime;
use dotenv::dotenv;
use rand::rngs::OsRng;
use rand::Rng;
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
    env::var("SITE_ID").expect("SITE_ID must be set")
}

pub fn get_jwt_secret() -> Vec<u8> {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
}

pub fn generate_secret(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill(&mut salt);
    salt
}

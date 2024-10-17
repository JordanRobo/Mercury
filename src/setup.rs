use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

fn generate_secret(length: usize) -> String {
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

pub fn run_setup() -> std::io::Result<()> {
    let jwt_secret = generate_secret(64);
    let site_id = generate_secret(32);

    let env_contents = format!("JWT_SECRET={}\nSITE_ID={}\nDATABASE_URL=mercury.db", jwt_secret, site_id);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".env")?;

    file.write_all(env_contents.as_bytes())?;

    println!("Setup complete. Environment file (.env) has been created with secure random values.");
    Ok(())
}

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "start" {
        mercury::setup::run_setup()?;
        println!("Setup completed. You can now start your application.");
        return Ok(());
    }

    // Check if .env file exists
    if !std::path::Path::new(".env").exists() {
        eprintln!("Error: Environment file (.env) not found.");
        eprintln!("Please run the start command to run server:");
        eprintln!("    ./mercury start");
        std::process::exit(1);
    }

    mercury::run()?.await
}

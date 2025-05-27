use cliclack::{intro, outro};
use console::style;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use mercury::users::{UserService, CreateUser};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/2024-07-04-034116_db_init");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "seed" { 
        intro(
            style(" Mercury Adding Mock Data to Database... ")
                .on_cyan()
                .black(),
        )?;

        outro("Data has been added successfully!")?;
        return Ok(());
    }

    if args.len() > 1 && args[1] == "start" {
        intro(
            style(" Mercury CMS Setup ")
                .on_cyan()
                .black(),
        )?;

        let first_name = cliclack::input("Welcome! Let's start off with your name:")
            .placeholder("Jordan")
            .required(true)
            .interact::<String>()?;

        let email = cliclack::input("Please enter your Admin accounts email address:")
            .placeholder("jordan@mercury.rs")
            .required(true)
            .interact::<String>()?;

        let password = cliclack::password("Lastly enter a password:")
            .mask('▪')
            .validate_interactively(|x: &String| if x.len() < 6 { Err("password should be at least 6 characters long") } else { Ok(()) })
            .interact()?;

        let pool = mercury::db::establish_connection();
        let conn = &mut pool.get().unwrap();

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        let admin = CreateUser {first_name, last_name: None, email, password};

        mercury::setup()?;
        UserService::new_admin(conn, admin).unwrap();

        outro("You're all set! Start your server with the './mercury' command")?;
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

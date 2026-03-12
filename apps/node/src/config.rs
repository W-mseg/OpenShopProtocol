use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub public_url: String,
    /// Argon2-hashed password, or raw if not yet hashed (migrated on first run)
    pub admin_password_hash: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        dotenvy::from_path(exe_dir.join(".env")).ok();
        dotenvy::dotenv().ok();

        let default_db = format!(
            "sqlite:{}",
            exe_dir.join("osp.db").to_string_lossy()
        );

        // Read raw password from env, hash it at startup
        let raw_password = env::var("ADMIN_PASSWORD")
            .unwrap_or_else(|_| "admin".into());

        let admin_password_hash = hash_password(&raw_password)?;

        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()?,
            database_url: env::var("DATABASE_URL").unwrap_or(default_db),
            public_url: env::var("PUBLIC_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            admin_password_hash,
        })
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };
    use rand_core::OsRng;
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("hash error: {e}"))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let Ok(parsed) = PasswordHash::new(hash) else { return false };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

use std::error::{self};

use argon2::Argon2;
use rand::{RngCore, SeedableRng};
use rusqlite::{fallible_streaming_iterator::FallibleStreamingIterator, Connection};

pub enum Storage {
    Memory,
    File(String),
}

pub struct Handler {
    conn: rusqlite::Connection
}

type LazyResult<T> = std::result::Result<T, Box<dyn error::Error>>;

impl Handler {

    pub fn new(storage: Storage) -> Result<Handler, rusqlite::Error> {
        // Build and initialize the user db
        let c = match storage {
            Storage::Memory => Connection::open_in_memory()?,
            Storage::File(path) => Connection::open(path)?,
        };
        {
            let mut stmt = c.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
            let rows = stmt.query(())?;
            if let Ok(0) = rows.count() {
                c.execute("CREATE TABLE passwords (
                        username TEXT PRIMARY KEY,
                        password BLOB NOT NULL,
                        salt BLOB NOT NULL)", ())?;
            }
        }

        Ok(Handler {
            conn: c
        })
    }

    fn generate_salt(&self) -> LazyResult<[u8;32]> {
        let mut rng = rand::rngs::StdRng::try_from_os_rng()?;
        let mut salt_buf = [0u8; 32];
        rng.fill_bytes(&mut salt_buf);
        Ok(salt_buf)
    }

    fn hash_password(&self, password: String, salt: [u8; 32]) -> LazyResult<[u8; 32]> {
        let argon2 = Argon2::default();
        let mut hash = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), &salt, &mut hash)?;
        Ok(hash)
    }

    pub fn register(&self, username: String, password: String) -> LazyResult<()> {
        let salt = self.generate_salt()?;
        self.conn.execute("INSERT INTO passwords (username, password, salt) VALUES (?1, ?2, ?3)", (username, self.hash_password(password, salt)?, salt))?;
        Ok(())
    }

    pub fn unregister(&self, username: String) -> LazyResult<()> {
        self.conn.execute("DELETE FROM passwords WHERE username = :username", rusqlite::named_params! {":username": username.as_str()})?;
        Ok(())
    }

    pub fn verify(&self, username: String, password: String) -> LazyResult<bool> {
        let mut stmt = self.conn.prepare("SELECT * FROM passwords WHERE username = :username")?;
        let res: Result<([u8; 32], [u8; 32]), rusqlite::Error> = stmt.query_row(rusqlite::named_params! {":username": username.as_str()}, |row| Ok((row.get(1)?, row.get(2)?)));
        match res {
            Ok((hashed_password, salt)) => {
                let hash = self.hash_password(password, salt)?;
                Ok(hashed_password == hash)
            },
            _ => Ok(false)
        }
    }
}
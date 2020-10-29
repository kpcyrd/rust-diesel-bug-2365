use anyhow::{Context, Result};
use diesel::connection::SimpleConnection;
use diesel::Connection;
use std::fs;
use std::path::Path;
use std::thread;

const THREADS: usize = 25;
const LOOPS: usize = 100;
const EXPECT: usize = THREADS * LOOPS;
const PATH: &str = "foo.db";
// ensure we execute the same query for both libs
const QUERY: &str = "UPDATE foo SET bar=bar+1;";

// setup stuff that sqlite needs
const SETUP: &str = "
PRAGMA journal_mode = WAL;          -- better write-concurrency
PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
PRAGMA busy_timeout = 1000;         -- sleep if the database is busy
PRAGMA foreign_keys = ON;           -- enforce foreign keys
";

trait Testable: Sync {
    fn setup(&self) -> Result<()> {
        if Path::new(PATH).exists() {
            println!("Old database still exists, removing");
            self.cleanup()?;
        }

        println!("Setting up database");
        let db = rusqlite::Connection::open(PATH)?;
        db.execute("CREATE TABLE foo (bar INTEGER);", rusqlite::NO_PARAMS)?;
        db.execute("INSERT INTO foo(bar) VALUES(0);", rusqlite::NO_PARAMS)?;
        db.close().map_err(|(_, err)| err)?;
        Ok(())
    }

    // it doesn't really matter which lib we use here,
    // we only test the database has the expected value
    fn verify(&self) -> Result<()> {
        let db = rusqlite::Connection::open(PATH)?;
        let f: i64 = db.query_row("SELECT bar FROM foo", rusqlite::NO_PARAMS, |r| r.get(0))?;
        if f as usize == EXPECT {
            println!("Verified database content successfully");
        } else {
            println!("Expected field to be: {}, but is actually: {}", EXPECT, f);
        }
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        fs::remove_file(PATH).ok();
        // print an empty line for readability
        println!();
        Ok(())
    }

    fn start(&self) -> Result<()> {
        self.setup()?;
        println!(
            "Spawning {} threads and increase a field by one {} times",
            THREADS, LOOPS
        );
        let mut threads = Vec::new();
        for _ in 0..THREADS {
            let t = thread::spawn(move || {
                if let Err(e) = Self::run() {
                    eprintln!("Error: {:#}", e);
                }
            });
            threads.push(t);
        }
        for t in threads {
            t.join().ok();
        }
        println!("All threads finished");
        self.verify()?;
        self.cleanup()?;
        Ok(())
    }

    fn run() -> Result<()>;
}

struct Rusqlite {}
impl Testable for Rusqlite {
    fn run() -> Result<()> {
        let db = rusqlite::Connection::open(PATH)?;
        db.execute_batch(SETUP)
            .context("Failed to execute PRAGMAs")?;
        for _ in 0..LOOPS {
            db.execute(QUERY, rusqlite::NO_PARAMS)
                .context("Failed to execute UPDATE")?;
        }
        Ok(())
    }
}

struct Diesel {}
impl Testable for Diesel {
    fn run() -> Result<()> {
        let db = diesel::SqliteConnection::establish(PATH)?;
        db.batch_execute(SETUP)
            .context("Failed to execute PRAGMAs")?;
        for _ in 0..LOOPS {
            db.execute(QUERY).context("Failed to execute UPDATE")?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let r = Rusqlite {};
    println!("Testing rusqlite");
    r.start()?;

    let d = Diesel {};
    println!("Testing diesel");
    d.start()?;

    Ok(())
}

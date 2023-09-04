use crate::{
    base64_encode, errors::GeyserStoreResult,
    update_account::UpdateAccountStorable,
};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

pub const DB_NAME: &str = "solana-geyser-updates.sqlite";

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new(path: &str) -> GeyserStoreResult<Self> {
        let conn = Connection::open(path)?;

        let db = Self { conn };
        db.init_tables()?;

        Ok(db)
    }

    // -----------------
    // Init Tables
    // -----------------
    pub fn init_tables(&self) -> GeyserStoreResult<()> {
        // TODO(thlorenz): dropping table should be configurable via CLI arg
        Ok(self.conn.execute_batch(
            "
BEGIN;
DROP TABLE IF EXISTS accounts;
CREATE TABLE IF NOT EXISTS accounts (
    id              TEXT PRIMARY KEY,    
    size            INTEGER,
    data            BLOB,
    slot            INTEGER,
    write_version   INTEGER,
    updated_at      INTEGER,
    pubkey          TEXT,
    owner           TEXT,
    lamports        INTEGER,
    executable      BOOLEAN,
    rent_epoch      TEXT
);
CREATE INDEX IF NOT EXISTS idx_pubkey ON accounts (pubkey);
CREATE INDEX IF NOT EXISTS idx_id ON accounts (id);
COMMIT;
",
        )?)
    }

    // -----------------
    // Insert Account Update
    // -----------------

    pub fn insert_account_update(
        &self,
        update: UpdateAccountStorable,
    ) -> GeyserStoreResult<usize> {
        let id = update.id();
        let now = SystemTime::now();
        let updated_at: u32 = time_stamp_to_secs(now);
        let size = update.data.len();
        let data = base64_encode(&update.data);
        Ok(self.conn.execute(
            "
INSERT OR IGNORE INTO accounts (
    id,    
    size,
    data,
    slot,
    write_version,
    updated_at,
    pubkey,
    owner,
    lamports,
    executable,
    rent_epoch
)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);
                ",
            params![
                id,
                size,
                data,
                update.slot,
                update.write_version,
                updated_at,
                update.pubkey,
                update.owner,
                update.lamports,
                update.executable,
                update.rent_epoch.to_string(),
            ],
        )?)
    }
}

// -----------------
// Sqlite helpers
// -----------------
fn time_stamp_to_secs(time_stamp: SystemTime) -> u32 {
    // max u32:          4,294,967,295
    // UNIX_EPOCH secs: ~1,631,804,843
    time_stamp.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32
}

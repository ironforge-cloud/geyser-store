use crate::{
    base64_encode, errors::GeyserStoreResult,
    notify_transaction::TransactionStorable,
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
    rent_epoch      INTEGER,
    txn_signature   TEXT
);
CREATE INDEX IF NOT EXISTS idx_pubkey ON accounts (pubkey);
CREATE INDEX IF NOT EXISTS idx_id ON accounts (id);

DROP TABLE IF EXISTS transactions;
CREATE TABLE IF NOT EXISTS transactions (
    signature                TEXT PRIMARY KEY,
    is_vote                  BOOLEAN,
    slot                     INTEGER,
    idx                      INTEGER,
    tx                       TEXT,
    transaction_status_meta  TEXT
);
CREATE INDEX IF NOT EXISTS idx_signature ON transactions (signature);

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
        let txn_signature = update.txn_signature.unwrap_or_default();
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
    rent_epoch,
    txn_signature
)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12);
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
                txn_signature
            ],
        )?)
    }

    // -----------------
    // Get All Accounts
    // -----------------
    pub fn get_all_accounts(
        &self,
    ) -> GeyserStoreResult<Vec<UpdateAccountStorable>> {
        let mut stmt = self.conn.prepare("SELECT * FROM accounts")?;
        let accounts_iter = stmt.query_map([], |row| {
            Ok(UpdateAccountStorable {
                slot: row.get("slot")?,
                pubkey: row.get("pubkey")?,
                lamports: row.get("lamports")?,
                owner: row.get("owner")?,
                executable: row.get("executable")?,
                // issues storing the rent epoch correctly apparently
                // it's stored as a huge number
                rent_epoch: 0, // row.get("rent_epoch")?,
                data: vec![],  // row.get("data")?,
                write_version: row.get("write_version")?,
                txn_signature: row.get("txn_signature")?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in accounts_iter {
            accounts.push(account?);
        }

        Ok(accounts)
    }

    // -----------------
    // Get Accounts By Owner
    // -----------------
    pub fn get_accounts_by_owner(
        &self,
        owner: &str,
    ) -> GeyserStoreResult<Vec<UpdateAccountStorable>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM accounts WHERE owner = ?1")?;
        let accounts_iter = stmt.query_map(params![owner], |row| {
            Ok(UpdateAccountStorable {
                slot: row.get("slot")?,
                pubkey: row.get("pubkey")?,
                lamports: row.get("lamports")?,
                owner: row.get("owner")?,
                executable: row.get("executable")?,
                // issues storing the rent epoch correctly apparently
                // it's stored as a huge number
                rent_epoch: 0, // row.get("rent_epoch")?,
                data: vec![],  // row.get("data")?,
                write_version: row.get("write_version")?,
                txn_signature: row.get("txn_signature")?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in accounts_iter {
            accounts.push(account?);
        }

        Ok(accounts)
    }
    // -----------------
    // Insert Transaction
    // -----------------
    pub fn insert_transaction(
        &self,
        tx: TransactionStorable,
    ) -> GeyserStoreResult<usize> {
        let signature = tx.signature;
        let is_vote = tx.is_vote;
        let slot = tx.slot;
        let index = tx.index;
        let transaction_data = format!("{:?}", tx.transaction);
        let transaction_status_meta =
            format!("{:?}", tx.transaction_status_meta);

        Ok(self.conn.execute(
            "
  INSERT OR IGNORE INTO transactions (
      signature,
      is_vote,
      slot,
      idx,
      tx,
      transaction_status_meta
  )
  VALUES (?1, ?2, ?3, ?4, ?5, ?6);
              ",
            params![
                signature,
                is_vote,
                slot,
                index,
                transaction_data,
                transaction_status_meta,
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

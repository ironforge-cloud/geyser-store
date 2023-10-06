use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use events::SerializableTransactionEvent;
use notify_transaction::TransactionStorable;

use crate::update_account::UpdateAccountStorable;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use update_account::UpdateAccount;

mod db;
pub mod errors;
mod events;
mod notify_transaction;
mod update_account;
mod utils;
pub use utils::*;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<db::DB>>,
}
async fn account_update(
    State(state): State<AppState>,
    item: Json<UpdateAccount>,
) -> (StatusCode, String) {
    let storable = UpdateAccountStorable::from(item.0);
    if std::env::var("DUMP_UPDATES").is_ok() {
        eprintln!("{:#?}", storable);
    }
    let db = state.db.lock().expect("mutex was poisoned");
    match db.insert_account_update(storable) {
        Ok(_) => (StatusCode::OK, "OK".to_string()),
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string())
        }
    }
}

async fn transaction_update(
    State(state): State<AppState>,
    item: Json<SerializableTransactionEvent>,
) -> (StatusCode, String) {
    let storable = TransactionStorable::from(item.0);
    if std::env::var("DUMP_UPDATES").is_ok() {
        eprintln!("{:#?}", storable);
    }
    let db = state.db.lock().expect("mutex was poisoned");
    match db.insert_transaction(storable) {
        Ok(_) => (StatusCode::OK, "OK".to_string()),
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let db = db::DB::new(db::DB_NAME).unwrap();
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };
    let app = Router::new()
        .route("/update-account", post(account_update))
        .route("/update-transaction", post(transaction_update))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

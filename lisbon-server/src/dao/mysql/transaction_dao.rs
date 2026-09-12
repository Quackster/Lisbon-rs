//! Mirrors `net.h4bbo.lisbon.dao.mysql.TransactionDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::item::transaction::Transaction;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct TransactionDao;

impl TransactionDao {
    /// Mirrors `createTransaction(int, String, String, int, String, int,
    /// int, boolean)` (the Java `SQLException` is swallowed by the Java
    /// callers).
    pub fn create_transaction(
        user_id: i32,
        item_id: &str,
        catalogue_id: &str,
        amount: i32,
        description: &str,
        credit_cost: i32,
        pixel_cost: i32,
        visible: bool,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_transactions (user_id, item_id, catalogue_id, amount, description, credit_cost, pixel_cost, is_visible) VALUES ({user_id}, '{}', '{}', {amount}, '{}', {credit_cost}, {pixel_cost}, {})",
            escape(item_id),
            escape(catalogue_id),
            escape(description),
            if visible { 1 } else { 0 }
        ));
    }

    /// Mirrors `getTransactionByItem(int)`.
    pub fn get_transaction_by_item(item_id: i32) -> Vec<Transaction> {
        let mut transactions = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_transactions WHERE item_id = {item_id} ORDER BY users_transactions.created_at DESC"
            ),
        ) {
            if let (Some(item_id), Some(description), Some(credit_cost), Some(pixel_cost), Some(amount), Some(created_at)) = (
                row.str("item_id"),
                row.str("description"),
                row.i32("credit_cost"),
                row.i32("pixel_cost"),
                row.i32("amount"),
                row.i64("created_at"),
            ) {
                let item_ids: Vec<String> = item_id.split(',').map(String::from).collect();
                transactions.push(Transaction::new(
                    &item_ids,
                    &description,
                    credit_cost,
                    pixel_cost,
                    amount,
                    created_at,
                ));
            }
        }

        transactions
    }

    /// Mirrors `getTransactionsPastMonth(String, boolean)`.
    pub fn get_transactions_past_month(search_query: &str, view_all: bool) -> Vec<Transaction> {
        let now = chrono::Local::now();
        let month = now.format("%m").to_string().parse::<i32>().unwrap_or(0);
        let year = now.format("%Y").to_string().parse::<i32>().unwrap_or(0);

        let mut transactions = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_transactions INNER JOIN users ON users.id = users_transactions.user_id WHERE MONTH(users_transactions.created_at) = {month} AND YEAR(users_transactions.created_at) = {year} AND user_id = {} OR username = '{}' ORDER BY users_transactions.created_at DESC",
                if search_query.chars().all(|c| c.is_ascii_digit()) {
                    search_query.parse::<i32>().unwrap_or(-1)
                } else {
                    -1
                },
                escape(search_query)
            ),
        ) {
            let is_visible = row.bool("is_visible").unwrap_or(false);

            if !is_visible && !view_all {
                continue;
            }

            if let (Some(item_id), Some(description), Some(credit_cost), Some(pixel_cost), Some(amount), Some(created_at)) = (
                row.str("item_id"),
                row.str("description"),
                row.i32("credit_cost"),
                row.i32("pixel_cost"),
                row.i32("amount"),
                row.i64("created_at"),
            ) {
                let item_ids: Vec<String> = item_id.split(',').map(String::from).collect();
                transactions.push(Transaction::new(
                    &item_ids,
                    &description,
                    credit_cost,
                    pixel_cost,
                    amount,
                    created_at,
                ));
            }
        }

        transactions
    }

    /// Mirrors `getTransactions(int, int, int, boolean)`.
    pub fn get_transactions(user_id: i32, month: i32, year: i32, view_all: bool) -> Vec<Transaction> {
        let mut transactions = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_transactions WHERE YEAR(created_at) = {year} AND MONTH(created_at) = {month} AND user_id = {user_id} ORDER BY created_at DESC"
            ),
        ) {
            let is_visible = row.bool("is_visible").unwrap_or(false);

            if !is_visible && !view_all {
                continue;
            }

            if let (Some(item_id), Some(description), Some(credit_cost), Some(pixel_cost), Some(amount), Some(created_at)) = (
                row.str("item_id"),
                row.str("description"),
                row.i32("credit_cost"),
                row.i32("pixel_cost"),
                row.i32("amount"),
                row.i64("created_at"),
            ) {
                let item_ids: Vec<String> = item_id.split(',').map(String::from).collect();
                transactions.push(Transaction::new(
                    &item_ids,
                    &description,
                    credit_cost,
                    pixel_cost,
                    amount,
                    created_at,
                ));
            }
        }

        transactions
    }
}

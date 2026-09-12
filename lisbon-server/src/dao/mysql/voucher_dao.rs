//! Mirrors `net.h4bbo.lisbon.dao.mysql.VoucherDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::misc::purse::voucher::Voucher;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct VoucherDao;

impl VoucherDao {
    /// Mirrors `redeemVoucher(String, int)`.
    pub fn redeem_voucher(voucher_code: &str, user_id: i32) -> Option<Voucher> {
        let mut voucher: Option<Voucher> = None;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT credits,is_single_use,allow_new_users FROM vouchers WHERE voucher_code = '{}' AND (expiry_date IS NULL OR (UNIX_TIMESTAMP() < UNIX_TIMESTAMP(expiry_date))) AND NOT EXISTS (SELECT vouchers_history.user_id FROM vouchers_history WHERE vouchers_history.user_id = {user_id} AND vouchers_history.voucher_code = '{}')",
                escape(voucher_code),
                escape(voucher_code)
            ),
        ) {
            if let (Some(credits), Some(is_single_use), Some(allow_new_users)) = (
                row.i32("credits"),
                row.bool("is_single_use"),
                row.bool("allow_new_users"),
            ) {
                let mut built = Voucher::new(credits, allow_new_users);

                for item_row in Storage::get_storage().query_all(
                    &format!(
                        "SELECT catalogue_sale_code FROM vouchers_items INNER JOIN catalogue_items ON catalogue_items.sale_code = vouchers_items.catalogue_sale_code WHERE voucher_code = '{}'",
                        escape(voucher_code)
                    ),
                ) {
                    if let Some(sale_code) = item_row.str("catalogue_sale_code") {
                        built.items.push(sale_code);
                    }
                }

                if is_single_use {
                    Storage::get_storage().execute(&format!(
                        "DELETE FROM vouchers WHERE voucher_code = '{}'",
                        escape(voucher_code)
                    ));
                    Storage::get_storage().execute(&format!(
                        "DELETE FROM vouchers_items WHERE voucher_code = '{}'",
                        escape(voucher_code)
                    ));
                }

                voucher = Some(built);
            }
        }

        voucher
    }

    /// Mirrors `logVoucher(String, int, int, List<CatalogueItem>)`.
    pub fn log_voucher(
        voucher_code: &str,
        user_id: i32,
        credits_redeemed: i32,
        items_redeemed: &[CatalogueItem],
    ) {
        let credits_value = if credits_redeemed > 0 {
            credits_redeemed.to_string()
        } else {
            "NULL".to_string()
        };

        let items_value = if items_redeemed.is_empty() {
            "NULL".to_string()
        } else {
            let mut distinct: Vec<(String, i32)> = Vec::new();

            for item in items_redeemed {
                let code = item.get_sale_code().to_string();

                if let Some(entry) = distinct.iter_mut().find(|(c, _)| *c == code) {
                    entry.1 += 1;
                } else {
                    distinct.push((code, 1));
                }
            }

            let mut builder = String::new();

            for (code, count) in &distinct {
                builder.push_str(count.to_string().as_str());
                builder.push(',');
                builder.push_str(code);
                builder.push('|');
            }

            builder.pop();

            format!("'{}'", escape(&builder))
        };

        Storage::get_storage().execute(&format!(
            "INSERT INTO vouchers_history (voucher_code, user_id, credits_redeemed, items_redeemed) VALUES ('{}', {user_id}, {credits_value}, {items_value})",
            escape(voucher_code)
        ));
    }
}

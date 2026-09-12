//! Mirrors `net.h4bbo.lisbon.game.misc.purse.Voucher`.

#[derive(Clone, Debug)]
pub struct Voucher {
    pub credits: i32,
    pub items: Vec<String>,
    allow_new_users: bool,
}

impl Voucher {
    /// Mirrors the 2-arg `Voucher(int, boolean)` constructor.
    pub fn new(credits: i32, allow_new_users: bool) -> Self {
        Self {
            credits,
            items: Vec::new(),
            allow_new_users,
        }
    }

    /// Mirrors `getCredits()`.
    pub fn get_credits(&self) -> i32 {
        self.credits
    }

    /// Mirrors `getItems()`.
    pub fn get_items(&self) -> Vec<String> {
        self.items.clone()
    }

    /// Mirrors `isAllowNewUsers()`.
    pub fn is_allow_new_users(&self) -> bool {
        self.allow_new_users
    }
}

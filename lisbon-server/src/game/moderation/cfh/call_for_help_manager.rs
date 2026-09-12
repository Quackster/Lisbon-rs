//! Mirrors `net.h4bbo.lisbon.game.moderation.cfh.CallForHelpManager`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::moderation::cfh::call_for_help::CallForHelp;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::moderation::call_for_help::CALL_FOR_HELP;
use crate::messages::outgoing::moderation::cry_received::CRY_RECEIVED;
use crate::messages::outgoing::moderation::delete_cry::DELETE_CRY;
use crate::messages::outgoing::moderation::picked_cry::PICKED_CRY;
use crate::messages::types::MessageComposer;
use crate::util::date_util::DateUtil;

pub struct CallForHelpManager {
    calls_for_help: Mutex<HashMap<i32, CallForHelp>>,
    latest_call_id: AtomicI32,
}

impl CallForHelpManager {
    /// Mirrors the no-arg constructor (the Java `ConcurrentHashMap` maps
    /// to a `parking_lot` guarded `HashMap`).
    pub fn new() -> Self {
        Self {
            calls_for_help: Mutex::new(HashMap::new()),
            latest_call_id: AtomicI32::new(0),
        }
    }

    /// Add a Call for Help to the queue.
    pub fn submit_call(&self, caller: &Player, message: &str) {
        let call_id = self.latest_call_id.fetch_add(1, Ordering::SeqCst);
        let caller_id = caller.get_details().get_id();
        let room = caller
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
            .unwrap_or_default();
        // Port note: Java NPEs when `getRoomUser()` / `getRoom()` is null.

        let cfh = CallForHelp::new(call_id, caller_id, room, message);
        self.calls_for_help.lock().insert(call_id, cfh.clone());

        self.send_to_moderators(&CALL_FOR_HELP::new(&cfh));
        caller.send(&CRY_RECEIVED);
    }

    /// Get a (call for help) by id.
    ///
    /// The Java map reference is returned by value (a `parking_lot` guard
    /// cannot outlive the call).
    pub fn get_call(&self, id: i32) -> Option<CallForHelp> {
        self.calls_for_help.lock().get(&id).cloned()
    }

    /// Get the open Call for Help for a user id.
    pub fn get_pending_call(&self, user_id: i32) -> Option<CallForHelp> {
        self.calls_for_help
            .lock()
            .values()
            .find(|cfh| cfh.is_open() && cfh.get_caller() == user_id)
            .cloned()
    }

    /// Get whether there is an open call for the player.
    pub fn has_pending_call(&self, player: &Player) -> bool {
        self.get_pending_call(player.get_details().get_id()).is_some()
    }

    /// Send a packet to all online moderators.
    fn send_to_moderators(&self, message: &dyn MessageComposer) {
        let players = PlayerManager::get_instance().get_players();
        for player_arc in players.iter() {
            let player = player_arc.lock();
            if player.has_fuse(&Fuseright::ReceiveCallsForHelp) {
                player.send(message);
            }
        }
    }

    /// Pick up a call for help.
    pub fn pick_up(&self, cfh: &CallForHelp, moderator: &Player) {
        let updated = {
            let mut calls = self.calls_for_help.lock();
            let Some(entry) = calls.get_mut(&cfh.get_cry_id()) else {
                return;
            };
            entry.set_picked_up_by(moderator);
            entry.clone()
        };

        self.send_to_moderators(&PICKED_CRY::new(&updated));
    }

    /// Change the category of a call.
    pub fn change_category(&self, cfh: &CallForHelp, new_category: i32) {
        let mut calls = self.calls_for_help.lock();
        let Some(entry) = calls.get_mut(&cfh.get_cry_id()) else {
            return;
        };
        entry.update_category(new_category);
        let updated = entry.clone();
        drop(calls);

        self.send_to_moderators(&CALL_FOR_HELP::new(&updated));
    }

    /// Delete the call for all moderators and mark it for deletion in 30
    /// minutes.
    pub fn delete_call(&self, cfh: &CallForHelp) {
        if let Some(entry) = self.calls_for_help.lock().get_mut(&cfh.get_cry_id()) {
            entry.set_deleted(true);
        }

        self.send_to_moderators(&DELETE_CRY::new(cfh.get_cry_id()));
    }

    /// Purge expired calls; the server remembers them for at least 30
    /// minutes.
    pub fn purge_expired_cfh(&self) {
        let now = DateUtil::get_current_time_seconds() as i64;

        let mut calls = self.calls_for_help.lock();
        let purged: Vec<i32> = calls
            .values()
            .filter(|cfh| !cfh.is_open() || now > cfh.get_expire_time())
            .map(|cfh| cfh.get_cry_id())
            .collect();

        for cry_id in purged {
            self.send_to_moderators(&DELETE_CRY::new(cry_id));
        }

        calls.retain(|_, cfh| cfh.is_open() && now <= cfh.get_expire_time());
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static CallForHelpManager {
        static INSTANCE: OnceLock<CallForHelpManager> = OnceLock::new();
        INSTANCE.get_or_init(CallForHelpManager::new)
    }
}

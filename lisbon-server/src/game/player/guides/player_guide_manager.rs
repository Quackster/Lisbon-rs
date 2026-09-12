//! Mirrors `net.h4bbo.lisbon.game.player.guides.PlayerGuideManager`.
// The Java `Player` back-reference is not held (a self-reference); the
// `player_id` is set once the player details are loaded so the
// `GuideDao`-backed lookups can run.

use parking_lot::{Mutex, MutexGuard};

use crate::dao::mysql::guide_dao::GuideDao;
use crate::game::player::guides::guiding_data::GuidingData;
use crate::game::player::player::Player;
use crate::messages::outgoing::guides::invitation::INVITATION;

#[derive(Default)]
pub struct PlayerGuideManager {
    invites: Mutex<Vec<i32>>,
    invited: Mutex<Vec<i32>>,
    guiding: Mutex<Vec<GuidingData>>,
    is_waiting_for_invitations: Mutex<bool>,
    is_waiting_for_guide: Mutex<bool>,
    started_for_waiting_guides_time: Mutex<i32>,
    has_tutorial: Mutex<bool>,
    is_guide: Mutex<bool>,
    is_guidable: Mutex<bool>,
    can_use_tutorial: Mutex<bool>,
    block_tutorial: Mutex<bool>,
    cancel_tutorial: Mutex<bool>,
    invited_by: Mutex<i32>,
    player_id: Mutex<i32>,
}

impl PlayerGuideManager {
    /// Mirrors the `PlayerGuideManager(Player)` constructor (the `Player`
    /// back-reference is inapplicable; the `GuidingData` list starts empty
    /// like the Java `CopyOnWriteArrayList`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `addInvite(Integer, String)` (the Java `Player`
    /// back-reference is passed explicitly, since a self-reference
    /// cannot be represented in Rust).
    pub fn add_invite(&self, player: &Player, user_id: i32, username: &str) {
        let mut invites = self.invites.lock();
        if invites.contains(&user_id) {
            return;
        }
        player.send(&INVITATION::new(user_id, username));
        invites.push(user_id);
    }

    /// Mirrors `hasInvite(int)`.
    pub fn has_invite(&self, id: i32) -> bool {
        self.invites.lock().contains(&id)
    }

    /// Mirrors `getInvites()`.
    pub fn get_invites(&self) -> Vec<i32> {
        self.invites.lock().clone()
    }

    /// Mirrors `removeInvite(int)`.
    pub fn remove_invite(&self, id: i32) {
        self.invites.lock().retain(|value| *value != id);
    }

    /// Mirrors `getInvites().clear()`.
    pub fn clear_invites(&self) {
        self.invites.lock().clear();
    }

    /// Mirrors `setWaitingForInvitations(boolean)`.
    pub fn set_waiting_for_invitations(&self, waiting: bool) {
        *self.is_waiting_for_invitations.lock() = waiting;
    }

    /// Mirrors `hasInvited(int)`.
    pub fn has_invited(&self, id: i32) -> bool {
        self.invited.lock().contains(&id)
    }

    /// Mirrors `addInvited(Integer)`.
    pub fn add_invited(&self, user_id: i32) {
        let mut invited = self.invited.lock();
        if invited.contains(&user_id) {
            return;
        }
        invited.push(user_id);
    }

    /// Mirrors `getInvited()`.
    pub fn get_invited(&self) -> MutexGuard<'_, Vec<i32>> {
        self.invited.lock()
    }

    /// Mirrors `isWaitingForInvitations()`.
    pub fn is_waiting_for_invitations(&self) -> bool {
        *self.is_waiting_for_invitations.lock()
    }

    /// Mirrors `getStartedForWaitingGuidesTime()`.
    pub fn get_started_for_waiting_guides_time(&self) -> i32 {
        *self.started_for_waiting_guides_time.lock()
    }

    /// Mirrors `setWaitingForGuide(boolean)`.
    pub fn set_waiting_for_guide(&self, waiting: bool) {
        *self.is_waiting_for_guide.lock() = waiting;
    }

    /// Mirrors `setStartedForWaitingGuidesTime(int)`.
    pub fn set_started_for_waiting_guides_time(&self, started: i32) {
        *self.started_for_waiting_guides_time.lock() = started;
    }

    /// Mirrors `isWaitingForGuide()`.
    pub fn is_waiting_for_guide(&self) -> bool {
        *self.is_waiting_for_guide.lock()
    }

    /// Mirrors `getInvitedBy()`.
    pub fn get_invited_by(&self) -> i32 {
        *self.invited_by.lock()
    }

    /// Mirrors `setInvitedBy(int)`.
    pub fn set_invited_by(&self, invited_by: i32) {
        *self.invited_by.lock() = invited_by;
    }

    /// Mirrors `isGuide()`.
    pub fn is_guide(&self) -> bool {
        *self.is_guide.lock()
    }

    /// Mirrors `setGuide(boolean)`.
    pub fn set_guide(&self, guide: bool) {
        *self.is_guide.lock() = guide;
    }

    /// Mirrors `isGuidable()`.
    pub fn is_guidable(&self) -> bool {
        *self.is_guidable.lock()
    }

    /// Mirrors `setGuidable(boolean)`.
    pub fn set_guidable(&self, guidable: bool) {
        *self.is_guidable.lock() = guidable;
    }

    /// Mirrors `hasTutorial()`.
    pub fn has_tutorial(&self) -> bool {
        *self.has_tutorial.lock()
    }

    /// Mirrors `setHasTutorial(boolean)`.
    pub fn set_has_tutorial(&self, has_tutorial: bool) {
        *self.has_tutorial.lock() = has_tutorial;
    }

    /// Mirrors `canUseTutorial()`.
    pub fn can_use_tutorial(&self) -> bool {
        *self.can_use_tutorial.lock()
    }

    /// Mirrors `setCanUseTutorial(boolean)`.
    pub fn set_can_use_tutorial(&self, can_use_tutorial: bool) {
        *self.can_use_tutorial.lock() = can_use_tutorial;
    }

    /// Mirrors `isBlockingTutorial()`.
    pub fn is_blocking_tutorial(&self) -> bool {
        *self.block_tutorial.lock()
    }

    /// Mirrors `setBlockingTutorial(boolean)`.
    pub fn set_blocking_tutorial(&self, block_tutorial: bool) {
        *self.block_tutorial.lock() = block_tutorial;
    }

    /// Mirrors `isCancelTutorial()`.
    pub fn is_cancel_tutorial(&self) -> bool {
        *self.cancel_tutorial.lock()
    }

    /// Mirrors `setCancelTutorial(boolean)`.
    pub fn set_cancel_tutorial(&self, cancel_tutorial: bool) {
        *self.cancel_tutorial.lock() = cancel_tutorial;
    }

    /// Mirrors `getGuiding()`.
    pub fn get_guiding(&self) -> MutexGuard<'_, Vec<GuidingData>> {
        self.guiding.lock()
    }

    /// Mirrors `refreshGuidingUsers()`.
    pub fn refresh_guiding_users(&self) {
        *self.guiding.lock() = GuideDao::get_guided_by(*self.player_id.lock());
    }

    /// Mirrors the Java live `player.getDetails().getId()` lookup (the
    /// `Player` back-reference is inapplicable).
    pub fn set_player_id(&self, player_id: i32) {
        *self.player_id.lock() = player_id;
    }
}

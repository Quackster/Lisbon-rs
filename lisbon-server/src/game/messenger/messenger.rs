//! Mirrors `net.h4bbo.lisbon.game.messenger.Messenger`.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::game::entity::entity::Entity;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::messenger::messenger_category::MessengerCategory;
use crate::game::messenger::messenger_message::MessengerMessage;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::player_rank::PlayerRank;
use crate::game::room::room::Room;
use crate::messages::outgoing::messenger::add_buddy::ADD_BUDDY;
use crate::messages::outgoing::messenger::friend_request::FRIEND_REQUEST;
use crate::messages::outgoing::messenger::friends_update::FRIENDS_UPDATE;
use crate::util::config::game_configuration::GameConfiguration;

/// Mirrors `Messenger`.
///
/// The Java `Player` back-reference is replaced by an explicit `player`
/// argument on the methods that send on it (re-locking the owner's
/// `Mutex<Player>` from within would deadlock, and a self-reference
/// cannot be represented in Rust).
#[derive(Clone)]
pub struct Messenger {
    official_status_update_speed: bool,
    user: MessengerUser,
    friends: Arc<Mutex<HashMap<i32, MessengerUser>>>,
    requests: Arc<Mutex<HashMap<i32, MessengerUser>>>,
    offline_messages: Arc<Mutex<HashMap<i32, MessengerMessage>>>,
    messenger_categories: Arc<Mutex<Vec<MessengerCategory>>>,
    friends_update: Arc<Mutex<Vec<MessengerUser>>>,
    friends_limit: i32,
    allows_friend_requests: bool,
    followed: Arc<Mutex<Option<Room>>>,
}

impl Messenger {
    /// Mirrors the `Messenger(Player)` constructor.
    pub fn from_player(player: &Player) -> Self {
        Self::from_details(player.get_details())
    }

    /// Mirrors the `Messenger(PlayerDetails)` constructor.
    pub fn from_details(details: &PlayerDetails) -> Self {
        let config = GameConfiguration::get_instance();
        let user = MessengerUser::from_details(details);

        let mut friends_limit = i32::MAX;

        if details.get_rank().unwrap_or(PlayerRank::Normal).rank_id() <= 1 {
            if details.has_club_subscription() {
                friends_limit = config.get_integer("messenger.max.friends.club");
            } else {
                friends_limit = config.get_integer("messenger.max.friends.nonclub");
            }
        }

        Self {
            official_status_update_speed: config
                .get_bool("messenger.enable.official.update.speed"),
            user,
            friends: Arc::new(Mutex::new(MessengerDao::get_friends(details.get_id()))),
            requests: Arc::new(Mutex::new(MessengerDao::get_requests(details.get_id()))),
            offline_messages: Arc::new(Mutex::new(MessengerDao::get_unread_messages(details.get_id()))),
            messenger_categories: Arc::new(Mutex::new(MessengerDao::get_categories(details.get_id()))),
            friends_update: Arc::new(Mutex::new(Vec::new())),
            friends_limit,
            allows_friend_requests: details.is_allow_friend_requests(),
            followed: Arc::new(Mutex::new(None)),
        }
    }

    /// Mirrors `sendStatusUpdate()` (the Java early-return on `user ==
    /// null` is inapplicable; the `user` is always present here).
    pub fn send_status_update(&self) {
        for friend in self.get_online_friends() {
            let friend = friend.lock();
            let Some(friend_messenger) = friend.get_messenger() else {
                continue;
            };

            if let Some(updated) = friend_messenger.get_friend(self.user.get_user_id()) {
                friend_messenger.queue_friend_update(&updated);
            }

            if !self.official_status_update_speed {
                friend.send(&FRIENDS_UPDATE::new(&friend, friend_messenger));
            }
        }
    }

    /// Mirrors `getOnlineFriends()`.
    fn get_online_friends(&self) -> Vec<Arc<Mutex<Player>>> {
        let mut friends = Vec::new();

        for user in self.friends.lock().values() {
            let Some(friend) = PlayerManager::get_instance().get_player_by_id(user.get_user_id())
            else {
                continue;
            };

            let is_mutual = {
                let friend_locked = friend.lock();
                friend_locked
                    .get_messenger()
                    .is_some_and(|m| m.has_friend(self.user.get_user_id()))
            };

            if is_mutual {
                friends.push(friend);
            }
        }

        friends
    }

    /// Mirrors `hasRequest(int)`.
    pub fn has_request(&self, user_id: i32) -> bool {
        self.get_request(user_id).is_some()
    }

    /// Mirrors `hasFriend(int)`.
    pub fn has_friend(&self, user_id: i32) -> bool {
        self.get_friend(user_id).is_some()
    }

    /// Mirrors `addFriend(MessengerUser)`.
    ///
    /// The final online-friend update degrades to a skip when the friend's
    /// lock is held (the `TUTOR` flow locks both players); the DB and
    /// local state still update.
    pub fn add_friend(&self, player: &Player, new_buddy: &MessengerUser) {
        if self.has_friend(new_buddy.get_user_id()) {
            return;
        }

        MessengerDao::remove_request(new_buddy.get_user_id(), self.user.get_user_id());
        MessengerDao::new_friend(self.user.get_user_id(), new_buddy.get_user_id());
        MessengerDao::new_friend(new_buddy.get_user_id(), self.user.get_user_id());

        if let Some(fresh) = PlayerDao::get_details(new_buddy.get_user_id()) {
            player.send(&ADD_BUDDY::new(player, MessengerUser::from_details(&fresh)));
        }

        self.requests.lock().remove(&new_buddy.get_user_id());
        self.friends.lock().insert(new_buddy.get_user_id(), new_buddy.clone());

        let me_as_buddy = self.user.clone();

        if let Some(friend) = PlayerManager::get_instance().get_player_by_id(new_buddy.get_user_id())
        {
            if let Some(friend) = friend.try_lock() {
                if let Some(friend_messenger) = friend.get_messenger() {
                    friend_messenger
                        .get_friends()
                        .insert(me_as_buddy.get_user_id(), me_as_buddy.clone());
                }

                friend.send(&ADD_BUDDY::new(&friend, me_as_buddy));
            }
        }
    }

    /// Mirrors `addRequest(MessengerUser)`.
    pub fn add_request(&self, requester: &MessengerUser) {
        MessengerDao::new_request(requester.get_user_id(), self.user.get_user_id());
        self.requests
            .lock()
            .insert(requester.get_user_id(), requester.clone());

        if let Some(requested) =
            PlayerManager::get_instance().get_player_by_id(self.user.get_user_id())
        {
            requested.lock().send(&FRIEND_REQUEST::new(requester.clone()));
        }
    }

    /// Mirrors `declineRequest(MessengerUser)`.
    pub fn decline_request(&self, requester: &MessengerUser) {
        MessengerDao::remove_request(requester.get_user_id(), self.user.get_user_id());
        self.requests.lock().remove(&requester.get_user_id());
    }

    /// Mirrors `declineAllRequests()`.
    pub fn decline_all_requests(&self) {
        MessengerDao::remove_all_requests(self.user.get_user_id());
        self.requests.lock().clear();
    }

    /// Mirrors `isFriendsLimitReached()`.
    pub fn is_friends_limit_reached(&self) -> bool {
        self.friends.lock().len() as i32 >= self.get_friends_limit()
    }

    /// Mirrors `getFriendsLimit()`.
    pub fn get_friends_limit(&self) -> i32 {
        self.friends_limit
    }

    /// Mirrors `getRequest(int)`.
    pub fn get_request(&self, user_id: i32) -> Option<MessengerUser> {
        self.requests.lock().get(&user_id).cloned()
    }

    /// Mirrors `getFriend(int)`.
    pub fn get_friend(&self, user_id: i32) -> Option<MessengerUser> {
        self.friends.lock().get(&user_id).cloned()
    }

    /// Mirrors `removeFriend(int)`.
    pub fn remove_friend(&self, user_id: i32) -> bool {
        self.friends.lock().remove(&user_id);

        MessengerDao::remove_friend(user_id, self.user.get_user_id());
        MessengerDao::remove_friend(self.user.get_user_id(), user_id);

        true
    }

    /// Mirrors `getOfflineMessages()`.
    pub fn get_offline_messages(&self) -> MutexGuard<'_, HashMap<i32, MessengerMessage>> {
        self.offline_messages.lock()
    }

    /// Mirrors `getFriends()`.
    pub fn get_friends(&self) -> MutexGuard<'_, HashMap<i32, MessengerUser>> {
        self.friends.lock()
    }

    /// Mirrors `getMessengerUser()`.
    pub fn get_messenger_user(&self) -> MessengerUser {
        self.user.clone()
    }

    /// Mirrors `getRequests()`.
    pub fn get_requests(&self) -> Vec<MessengerUser> {
        self.requests.lock().values().cloned().collect()
    }

    /// Mirrors `allowsFriendRequests()`.
    pub fn allows_friend_requests(&self) -> bool {
        self.allows_friend_requests
    }

    /// Mirrors `getFriendsUpdate()` (the Java `BlockingQueue`, drained by
    /// `FRIENDS_UPDATE`).
    pub fn get_friends_update(&self) -> Vec<MessengerUser> {
        self.friends_update.lock().drain(..).collect()
    }

    /// Mirrors `queueFriendUpdate(MessengerUser)`.
    pub fn queue_friend_update(&self, friend: &MessengerUser) {
        let mut queue = self.friends_update.lock();
        queue.retain(|queued| queued.get_user_id() != friend.get_user_id());
        queue.push(friend.clone());
    }

    /// Mirrors `getCategories()`.
    pub fn get_categories(&self) -> Vec<MessengerCategory> {
        self.messenger_categories.lock().clone()
    }

    /// Mirrors `getCategories().clear(); getCategories().addAll(
    /// MessengerDao.getCategories(userId))` (the `REFRESH_MESSENGER_CATEGORIES`
    /// RCON branch).
    pub fn reload_categories(&self, user_id: i32) {
        let mut categories = self.messenger_categories.lock();
        categories.clear();
        categories.extend(MessengerDao::get_categories(user_id));
    }

    /// Mirrors `hasFollowed(Room)`.
    pub fn has_followed(&self, friend_room: &Room) {
        *self.followed.lock() = Some(friend_room.clone());
    }

    /// Mirrors `getFollowed()`.
    pub fn get_followed(&self) -> Option<Room> {
        self.followed.lock().clone()
    }
}

impl serde::Serialize for Messenger {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let friends = self.friends.lock();
        let requests = self.requests.lock();
        let offline = self.offline_messages.lock();
        let categories = self.messenger_categories.lock();
        let update = self.friends_update.lock();
        let followed = self.followed.lock();

        #[derive(serde::Serialize)]
        struct MessengerView<'a> {
            user: &'a MessengerUser,
            friends: Vec<&'a MessengerUser>,
            requests: Vec<&'a MessengerUser>,
            offline_messages: Vec<&'a MessengerMessage>,
            messenger_categories: Vec<&'a MessengerCategory>,
            friends_update: Vec<&'a MessengerUser>,
            friends_limit: i32,
            allows_friend_requests: bool,
            followed: Option<&'a Room>,
        }

        MessengerView {
            user: &self.user,
            friends: friends.values().collect(),
            requests: requests.values().collect(),
            offline_messages: offline.values().collect(),
            messenger_categories: categories.iter().collect(),
            friends_update: update.iter().collect(),
            friends_limit: self.friends_limit,
            allows_friend_requests: self.allows_friend_requests,
            followed: followed.as_ref(),
        }
        .serialize(serializer)
    }
}

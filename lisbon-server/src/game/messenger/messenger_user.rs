//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerUser`.

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::server::netty::streams::NettyResponse;
use crate::util::date_util::{DateUtil, LONG_DATE};
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct MessengerUser {
    user_id: i32,
    username: String,
    figure: String,
    sex: String,
    motto: String,
    last_online: i64,
    allow_stalking: bool,
    to_remove: bool,
    to_add: bool,
    category_id: i32,
    online_status_visible: bool,
    is_online: bool,
}

impl MessengerUser {
    /// Mirrors the `MessengerUser(PlayerDetails)` constructor (the category
    /// id defaults to 0, mirroring the Java).
    pub fn from_details(details: &PlayerDetails) -> Self {
        Self::apply_user_details(
            details.get_id(),
            details.get_name(),
            details.get_figure(),
            details.get_motto(),
            details.get_sex(),
            details.get_last_online(),
            details.does_allow_stalking(),
            0,
            details.is_online(),
            details.is_online_status_visible(),
        )
    }

    /// Mirrors the 10-arg `MessengerUser` constructor.
    pub fn new(
        user_id: i32,
        username: &str,
        figure: &str,
        sex: &str,
        console_motto: &str,
        last_online: i64,
        allow_stalking: bool,
        category_id: i32,
        is_online: bool,
        online_status_visible: bool,
    ) -> Self {
        Self::apply_user_details(
            user_id,
            username,
            figure,
            console_motto,
            sex,
            last_online,
            allow_stalking,
            category_id,
            is_online,
            online_status_visible,
        )
    }

    /// Mirrors `applyUserDetails` (generic method used by both
    /// constructors).
    fn apply_user_details(
        user_id: i32,
        username: &str,
        figure: &str,
        console_motto: &str,
        sex: &str,
        last_online: i64,
        allow_stalking: bool,
        category_id: i32,
        is_online: bool,
        online_status_visible: bool,
    ) -> Self {
        Self {
            to_remove: false,
            user_id,
            username: StringUtil::filter_input(username, true),
            figure: StringUtil::filter_input(figure, true),
            sex: if sex.to_lowercase() == "f" {
                "F".to_string()
            } else {
                "M".to_string()
            },
            last_online,
            motto: StringUtil::filter_input(console_motto, true),
            allow_stalking,
            to_add: false,
            category_id,
            online_status_visible,
            is_online,
        }
    }

    /// Mirrors `serialise(Player, NettyResponse)` (serialises the player,
    /// used for console search and the friends list).
    pub fn serialise(&mut self, friend: &Player, response: &mut NettyResponse) {
        if let Some(player_arc) = PlayerManager::get_instance().get_player_by_id(self.user_id) {
            let player = player_arc.lock();
            let details = player.get_details();
            self.figure = details.get_figure().to_string();
            self.last_online = details.get_last_online();
            self.sex = details.get_sex().to_string();
            self.motto = details.get_motto().to_string();
            self.allow_stalking = details.does_allow_stalking();
        }

        let is_online = PlayerManager::get_instance().is_player_online(self.user_id);

        response.write_int(self.user_id);
        response.write_string(self.username.as_str());
        response.write_bool(self.sex.to_lowercase() == "m");
        response.write_bool(is_online);
        response.write_bool(self.can_follow_friend(friend));
        response.write_string(if is_online {
            self.figure.as_str()
        } else {
            ""
        });
        response.write_int(self.category_id);
        response.write_string(self.motto.as_str());
        response.write_string(DateUtil::get_date(self.last_online, LONG_DATE));
    }

    /// Mirrors `canFollowFriend`.
    ///
    /// The Java NPE sites (`getRoomUser()` / `getRoom()` /
    /// `getRoom().getModel()` null) degrade to early `false` returns.
    pub fn can_follow_friend(&self, _friend: &Player) -> bool {
        let Some(player_arc) = PlayerManager::get_instance().get_player_by_id(self.user_id) else {
            return false;
        };

        let player = player_arc.lock();
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return false;
        };

        let Some(model) = room.get_model() else {
            return false;
        };

        let name = model.get_name();
        !name.starts_with("bb_") && name != "snowwar"
    }

    /// Mirrors `getUserId`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getUsername`.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Mirrors `setUsername`.
    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    /// Mirrors `getFigure`.
    pub fn get_figure(&self) -> &str {
        &self.figure
    }

    /// Mirrors `setFigure`.
    pub fn set_figure(&mut self, figure: &str) {
        self.figure = figure.to_string();
    }

    /// Mirrors `getSex`.
    pub fn get_sex(&self) -> &str {
        &self.sex
    }

    /// Mirrors `getMotto`.
    pub fn get_motto(&self) -> &str {
        &self.motto
    }

    /// Mirrors `setMotto`.
    pub fn set_motto(&mut self, motto: &str) {
        self.motto = motto.to_string();
    }

    /// Mirrors `getFormattedLastOnline`.
    pub fn get_formatted_last_online(&self) -> String {
        DateUtil::get_date(self.last_online, "dd/MM/yyyy hh:mm a")
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getFormatLastOnline`.
    pub fn get_format_last_online(&self, format: &str) -> String {
        DateUtil::get_date(self.last_online, format)
    }

    /// Mirrors `getLastOnline`.
    pub fn get_last_online(&self) -> i64 {
        self.last_online
    }

    /// Mirrors `setLastOnline`.
    pub fn set_last_online(&mut self, last_online: i64) {
        self.last_online = last_online;
    }

    /// Mirrors `removed`.
    pub fn removed(&self) -> bool {
        self.to_remove
    }

    /// Mirrors `setToRemove`.
    pub fn set_to_remove(&mut self, to_remove: bool) {
        self.to_remove = to_remove;
    }

    /// Mirrors `added`.
    pub fn added(&self) -> bool {
        self.to_add
    }

    /// Mirrors `setToAdd`.
    pub fn set_to_add(&mut self, to_add: bool) {
        self.to_add = to_add;
    }

    /// Mirrors `isOnline`.
    pub fn is_online(&self) -> bool {
        if !self.online_status_visible {
            return false;
        }

        if PlayerManager::get_instance().get_players().len() > 0 {
            return PlayerManager::get_instance().is_player_online(self.user_id);
        }

        self.is_online
    }

    /// Mirrors `getCategoryId`.
    pub fn get_category_id(&self) -> i32 {
        self.category_id
    }

    /// Mirrors `setCategoryId`.
    pub fn set_category_id(&mut self, category_id: i32) {
        self.category_id = category_id;
    }
}

impl std::fmt::Display for MessengerUser {
    /// Mirrors `toString`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{},{},{},{},{}]",
            self.username, self.motto, self.figure, self.sex, self.last_online
        )
    }
}

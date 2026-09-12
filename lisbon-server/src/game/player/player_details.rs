//! Mirrors `net.h4bbo.lisbon.game.player.PlayerDetails`.

use crate::dao::mysql::ban_dao::BanDao;
use crate::dao::mysql::group_dao::GroupDao;
use crate::dao::mysql::group_member_dao::GroupMemberDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::ban::ban_type::BanType;
use crate::game::groups::group_member::GroupMember;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::player_rank::PlayerRank;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::{DateUtil, SHORT_DATE};
use crate::util::string_util::StringUtil;

#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PlayerDetails {
    id: i32,
    username: String,
    email: String,
    figure: String,
    pool_figure: String,
    motto: String,
    sex: String,
    sso_ticket: String,
    machine_id: String,

    credits: i32,
    pixels: i32,
    last_pixels_time: i64,
    tickets: i32,
    film: i32,
    rank: Option<PlayerRank>,

    first_club_subscription: i64,
    club_expiration: i64,

    allow_stalking: bool,
    selected_room_id: i32,
    allow_friend_requests: bool,
    online_status_visible: bool,
    profile_visible: bool,
    word_filter_enabled: bool,
    trade_enabled: bool,
    sound_enabled: bool,

    next_handout: i64,
    last_online: i64,
    join_date: i64,

    is_online: bool,
    created_at: String,
    favourite_group_id: i32,
    receive_news: bool,
    trade_ban_expiration: i64,
    birthday: String,
    group_member: Option<GroupMember>,
}

impl PlayerDetails {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors the 31-arg `fill` overload (the 15-minute `lastPixelsTime`
    /// offset is `TimeUnit.MINUTES.toSeconds(15)`).
    pub fn fill(
        &mut self,
        id: i32,
        username: &str,
        figure: &str,
        pool_figure: &str,
        credits: i32,
        email: &str,
        motto: &str,
        sex: &str,
        sso_ticket: &str,
        tickets: i32,
        film: i32,
        rank: i32,
        last_online: i64,
        join_date: i64,
        machine_id: &str,
        first_club_subscription: i64,
        club_expiration: i64,
        allow_stalking: bool,
        selected_room: i32,
        allow_friend_requests: bool,
        online_status_visible: bool,
        profile_visible: bool,
        word_filter_enabled: bool,
        trade_enabled: bool,
        sound_enabled: bool,
        trade_ban_expiration: i64,
        receive_news: bool,
        is_online: bool,
        favourite_group_id: i32,
        created_at: &str,
    ) {
        self.id = id;
        self.username = StringUtil::filter_input(username, true);
        self.figure = StringUtil::filter_input(figure, true);
        self.pool_figure = StringUtil::filter_input(pool_figure, true);
        self.motto =
            crate::game::wordfilter::wordfilter_manager::WordfilterManager::filter_sentence(
                &StringUtil::filter_input(motto, true),
            );
        self.email = email.to_string();
        self.sex = if sex.to_lowercase() == "f" {
            "F".to_string()
        } else {
            "M".to_string()
        };
        self.sso_ticket = sso_ticket.to_string();
        self.last_pixels_time = DateUtil::get_current_time_seconds() as i64 + 15 * 60;
        self.credits = credits;
        self.tickets = tickets;
        self.film = film;
        self.rank = PlayerRank::get_rank_for_id(rank);
        self.last_online = last_online;
        self.join_date = join_date;
        self.machine_id = machine_id.to_string();
        self.first_club_subscription = first_club_subscription;
        self.club_expiration = club_expiration;
        self.allow_stalking = allow_stalking;
        self.selected_room_id = selected_room;
        self.allow_friend_requests = allow_friend_requests;
        self.online_status_visible = online_status_visible;
        self.profile_visible = profile_visible;
        self.word_filter_enabled = word_filter_enabled;
        self.trade_enabled = trade_enabled;
        self.sound_enabled = sound_enabled;
        self.trade_ban_expiration = trade_ban_expiration;
        self.is_online = is_online;
        self.favourite_group_id = favourite_group_id;
        self.receive_news = receive_news;
        self.created_at = created_at.to_string();

        if self.credits < 0 {
            self.credits = 0;
        }

        if self.tickets < 0 {
            self.tickets = 0;
        }

        if self.film < 0 {
            self.film = 0;
        }
    }

    /// Mirrors the 5-arg `fill` overload.
    pub fn fill_brief(&mut self, id: i32, username: &str, figure: &str, motto: &str, sex: &str) {
        self.id = id;
        self.username = username.to_string();
        self.figure = figure.to_string();
        self.motto = motto.to_string();
        self.sex = sex.to_string();
        self.created_at = String::new();
    }

    /// Materialises a `PlayerDetails` from the JSON a template value holds
    /// (mirrors the Java `(PlayerDetails) template.get("playerDetails")`
    /// cast; the template serialised the object when it was assigned).
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }

    /// Mirrors `hasClubSubscription`.
    pub fn has_club_subscription(&self) -> bool {
        if self.club_expiration != 0 {
            if (DateUtil::get_current_time_seconds() as i64) < self.club_expiration {
                return true;
            }
        }

        false
    }

    /// Mirrors `isBanned` — Java `Pair<String, Long>` maps to
    /// `Option<(String, i64)>`.
    pub fn is_banned(&self) -> Option<(String, i64)> {
        let user_ban_check = BanDao::has_ban(BanType::UserId, &self.id.to_string());
        if user_ban_check.is_some() {
            return user_ban_check;
        }

        let machine_ban_check = BanDao::has_ban(BanType::MachineId, &self.machine_id);
        if machine_ban_check.is_some() {
            return machine_ban_check;
        }

        let ip_ban_check = BanDao::has_ban(
            BanType::IpAddress,
            &PlayerDao::get_latest_ip(self.id),
        );
        if ip_ban_check.is_some() {
            return ip_ban_check;
        }

        None
    }

    /// Mirrors `resetNextHandout`.
    pub fn reset_next_handout(&mut self) {
        let config = GameConfiguration::get_instance();

        if config.get_integer("daily.credits.amount") > 0 {
            self.next_handout = 0;
        } else {
            let interval = config.get_integer("credits.scheduler.interval") as i64;
            let seconds = match config.get_string("credits.scheduler.timeunit").to_uppercase().as_str() {
                "NANOSECONDS" => interval / 1_000_000_000,
                "MICROSECONDS" => interval / 1_000_000,
                "MILLISECONDS" => interval / 1_000,
                "SECONDS" => interval,
                "MINUTES" => interval * 60,
                "HOURS" => interval * 3_600,
                "DAYS" => interval * 86_400,
                _ => interval,
            };

            self.next_handout = DateUtil::get_current_time_seconds() as i64 + seconds;
        }
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.username
    }

    /// Mirrors `getFigure`.
    pub fn get_figure(&self) -> &str {
        &self.figure
    }

    /// Mirrors `setFigure`.
    pub fn set_figure(&mut self, figure: &str) {
        self.figure = figure.to_string();
    }

    /// Mirrors `getPoolFigure`.
    pub fn get_pool_figure(&self) -> &str {
        &self.pool_figure
    }

    /// Mirrors `setPoolFigure`.
    pub fn set_pool_figure(&mut self, pool_figure: &str) {
        self.pool_figure = pool_figure.to_string();
    }

    /// Mirrors `getCredits`.
    pub fn get_credits(&self) -> i32 {
        self.credits
    }

    /// Mirrors `setCredits`.
    pub fn set_credits(&mut self, credits: i32) {
        self.credits = credits;
    }

    /// Getter for the `pixels` field (no Java counterpart; the field is
    /// otherwise write-only in `PlayerDetails`).
    pub fn get_pixels(&self) -> i32 {
        self.pixels
    }

    /// Getter for the `lastPixelsTime` field (no Java counterpart; the field
    /// is otherwise write-only in `PlayerDetails`).
    pub fn get_last_pixels_time(&self) -> i64 {
        self.last_pixels_time
    }

    /// Mirrors `getMotto`.
    pub fn get_motto(&self) -> &str {
        &self.motto
    }

    /// Mirrors `setMotto`.
    pub fn set_motto(&mut self, motto: &str) {
        self.motto = motto.to_string();
    }

    /// Mirrors `getSex`.
    pub fn get_sex(&self) -> &str {
        &self.sex
    }

    /// Mirrors `setSex`.
    pub fn set_sex(&mut self, sex: &str) {
        self.sex = sex.to_string();
    }

    /// Mirrors `getTickets`.
    pub fn get_tickets(&self) -> i32 {
        self.tickets
    }

    /// Mirrors `setTickets`.
    pub fn set_tickets(&mut self, tickets: i32) {
        self.tickets = tickets;
    }

    /// Mirrors `getFilm`.
    pub fn get_film(&self) -> i32 {
        self.film
    }

    /// Mirrors `setFilm`.
    pub fn set_film(&mut self, film: i32) {
        self.film = film;
    }

    /// Mirrors `getRank`.
    pub fn get_rank(&self) -> Option<PlayerRank> {
        self.rank
    }

    /// Mirrors `setRank`.
    pub fn set_rank(&mut self, rank: Option<PlayerRank>) {
        self.rank = rank;
    }

    /// Mirrors `getLastOnline`.
    pub fn get_last_online(&self) -> i64 {
        self.last_online
    }

    /// Mirrors `setLastOnline`.
    pub fn set_last_online(&mut self, last_online: i64) {
        self.last_online = last_online;
    }

    /// Mirrors `getFirstClubSubscription`.
    pub fn get_first_club_subscription(&self) -> i64 {
        self.first_club_subscription
    }

    /// Mirrors `setFirstClubSubscription`.
    pub fn set_first_club_subscription(&mut self, first_club_subscription: i64) {
        self.first_club_subscription = first_club_subscription;
    }

    /// Mirrors `getClubExpiration`.
    pub fn get_club_expiration(&self) -> i64 {
        self.club_expiration
    }

    /// Mirrors `setClubExpiration`.
    pub fn set_club_expiration(&mut self, club_expiration: i64) {
        self.club_expiration = club_expiration;
    }

    /// Mirrors `doesAllowStalking`.
    pub fn does_allow_stalking(&self) -> bool {
        self.allow_stalking
    }

    /// Mirrors `setAllowStalking`.
    pub fn set_allow_stalking(&mut self, allow_stalking: bool) {
        self.allow_stalking = allow_stalking;
    }

    /// Mirrors `getSoundSetting`.
    pub fn get_sound_setting(&self) -> bool {
        self.sound_enabled
    }

    /// Mirrors `setSoundSetting`.
    pub fn set_sound_setting(&mut self, sound_enabled: bool) {
        self.sound_enabled = sound_enabled;
    }

    /// Mirrors `getNextHandout`.
    pub fn get_next_handout(&self) -> i64 {
        self.next_handout
    }

    /// Mirrors `setNextHandout`.
    pub fn set_next_handout(&mut self, next_handout: i64) {
        self.next_handout = next_handout;
    }

    /// Mirrors `isAllowFriendRequests`.
    pub fn is_allow_friend_requests(&self) -> bool {
        self.allow_friend_requests
    }

    /// Mirrors `getSsoTicket`.
    pub fn get_sso_ticket(&self) -> &str {
        &self.sso_ticket
    }

    /// Mirrors `setSsoTicket`.
    pub fn set_sso_ticket(&mut self, sso_ticket: &str) {
        self.sso_ticket = sso_ticket.to_string();
    }

    /// Mirrors `getMachineId`.
    pub fn get_machine_id(&self) -> &str {
        &self.machine_id
    }

    /// Mirrors `setMachineId`.
    pub fn set_machine_id(&mut self, machine_id: &str) {
        self.machine_id = machine_id.to_string();
    }

    /// Mirrors `canSelectRoom`.
    pub fn can_select_room(&self) -> bool {
        self.selected_room_id == 0
    }

    /// Mirrors `hasSelectedRoom`.
    pub fn has_selected_room(&self) -> bool {
        self.selected_room_id > 0
    }

    /// Mirrors `getSelectedRoomId`.
    pub fn get_selected_room_id(&self) -> i32 {
        self.selected_room_id
    }

    /// Mirrors `setSelectedRoomId`.
    pub fn set_selected_room_id(&mut self, selected_room_id: i32) {
        self.selected_room_id = selected_room_id;
    }

    /// Mirrors `getEmail`.
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Mirrors `setEmail`.
    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    /// Mirrors `isOnline` (hidden when the online status is not visible).
    pub fn is_online(&self) -> bool {
        if self.is_online_status_visible() {
            return self.is_online;
        }

        false
    }

    /// Mirrors `getFormattedLastOnline`.
    pub fn get_formatted_last_online(&self) -> String {
        DateUtil::get_date(self.last_online, SHORT_DATE)
    }

    /// Mirrors `formatLastOnline`.
    pub fn format_last_online(&self, format: &str) -> String {
        DateUtil::get_date(self.last_online, format)
    }

    /// Mirrors `formatJoinDate`.
    pub fn format_join_date(&self, format: &str) -> String {
        DateUtil::get_date(self.join_date, format)
    }

    /// Mirrors `getCreatedAt` (first space-separated token).
    pub fn get_created_at(&self) -> &str {
        self.created_at.split(' ').next().unwrap_or("")
    }

    /// Mirrors `getJoinDate`.
    pub fn get_join_date(&self) -> i64 {
        self.join_date
    }

    /// Mirrors `isOnlineStatusVisible`.
    pub fn is_online_status_visible(&self) -> bool {
        self.online_status_visible
    }

    /// Mirrors `setOnlineStatusVisible`.
    pub fn set_online_status_visible(&mut self, online_status_visible: bool) {
        self.online_status_visible = online_status_visible;
    }

    /// Mirrors `isProfileVisible`.
    pub fn is_profile_visible(&self) -> bool {
        self.profile_visible
    }

    /// Mirrors `isWordFilterEnabled`.
    pub fn is_word_filter_enabled(&self) -> bool {
        self.word_filter_enabled
    }

    /// Mirrors `isTradeEnabled`.
    pub fn is_trade_enabled(&self) -> bool {
        self.trade_enabled
    }

    /// Mirrors `setTradeEnabled`.
    pub fn set_trade_enabled(&mut self, trade_enabled: bool) {
        self.trade_enabled = trade_enabled;
    }

    /// Mirrors `getTradeBanExpiration`.
    pub fn get_trade_ban_expiration(&self) -> i64 {
        self.trade_ban_expiration
    }

    /// Mirrors `setTradeBanExpiration`.
    pub fn set_trade_ban_expiration(&mut self, trade_ban_expiration: i64) {
        self.trade_ban_expiration = trade_ban_expiration;
    }

    /// Mirrors `isTradeBanned`.
    pub fn is_trade_banned(&self) -> bool {
        if self.trade_ban_expiration > 0 {
            return self.trade_ban_expiration > DateUtil::get_current_time_seconds() as i64;
        }

        false
    }

    /// Mirrors `getFavouriteGroupId`.
    pub fn get_favourite_group_id(&self) -> i32 {
        self.favourite_group_id
    }

    /// Mirrors `setFavouriteGroupId`.
    pub fn set_favourite_group_id(&mut self, favourite_group_id: i32) {
        self.favourite_group_id = favourite_group_id;
    }

    /// Mirrors `getGroupMember`.
    pub fn get_group_member(&mut self) -> Option<&GroupMember> {
        if self.group_member.is_none() && self.favourite_group_id > 0 {
            let group = if let Some(player) =
                PlayerManager::get_instance().get_player_by_id(self.id)
            {
                match player.try_lock() {
                    Some(guard) => guard
                        .get_joined_groups()
                        .and_then(|groups| {
                            groups
                                .iter()
                                .find(|g| g.get_id() == self.favourite_group_id)
                                .cloned()
                        }),
                    // The caller holds this player's lock; use the database
                    // lookup (the Java offline branch) instead of deadlocking.
                    None => GroupDao::get_group(self.favourite_group_id),
                }
            } else {
                GroupDao::get_group(self.favourite_group_id)
            };

            if group.is_none() {
                self.favourite_group_id = 0;
                PlayerDao::save_favourite_group(self.id, 0);
            } else if group.as_ref().unwrap().get_owner_id() == self.id {
                self.group_member = Some(GroupMember::new(
                    self.id,
                    self.favourite_group_id,
                    false,
                    3,
                ));
            } else {
                self.group_member =
                    GroupMemberDao::get_member(self.favourite_group_id, self.id);
            }
        }

        self.group_member.as_ref()
    }

    /// Mirrors `getIpAddress`.
    pub fn get_ip_address(&self) -> String {
        crate::dao::mysql::player_dao::PlayerDao::get_latest_ip(self.id)
    }

    /// Mirrors `isReceiveNews`.
    pub fn is_receive_news(&self) -> bool {
        self.receive_news
    }

    /// Mirrors `setReceiveNews`.
    pub fn set_receive_news(&mut self, receive_news: bool) {
        self.receive_news = receive_news;
    }

    /// Mirrors `getBirthday`.
    pub fn get_birthday(&self) -> &str {
        &self.birthday
    }

    /// Mirrors `setBirthday`.
    pub fn set_birthday(&mut self, birthday: &str) {
        self.birthday = birthday.to_string();
    }
}

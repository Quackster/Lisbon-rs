//! Mirrors `net.h4bbo.lisbon.dao.mysql.PlayerDao`.

use sqlx::mysql::MySqlRow;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

const FIGURE_BLACKLIST_1: &str = "hd-180-1.hr-100-61.ch-210-66.lg-270-82.sh-290-80";

pub struct PlayerDao;

impl PlayerDao {
    /// Mirrors `resetOnline()`.
    pub fn reset_online() {
        Storage::get_storage()
            .execute("UPDATE users SET is_online = 0 WHERE is_online = 1");
    }

    /// Mirrors `countIpAddress(String)`.
    pub fn count_ip_address(ip_address: &str) -> i32 {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(DISTINCT(user_id)) as count FROM users_ip_logs WHERE ip_address = '{}'",
                escape(ip_address)
            ),
        ) {
            if let Some(value) = row.i32("count") {
                return value;
            }
        }

        0
    }

    /// Mirrors `logIpAddress(int, String)`.
    pub fn log_ip_address(user_id: i32, ip_address: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_ip_logs (user_id, ip_address) VALUES ({user_id}, '{}')",
            escape(ip_address)
        ));
    }

    /// Mirrors `getIpAddressAt(int, int, int)`.
    pub fn get_ip_address_at(user_id: i32, position: i32, max_rows: i32) -> Option<String> {
        for (index, row) in Storage::get_storage()
            .query_all(&format!(
                "SELECT ip_address FROM users_ip_logs WHERE user_id = {user_id} ORDER BY created_at DESC LIMIT {max_rows}"
            ))
            .into_iter()
            .enumerate()
        {
            if index == position as usize {
                if let Some(value) = row.str("ip_address") {
                    return Some(value);
                }
            }
        }

        None
    }

    /// Mirrors `getIpAddresses(int, int)`.
    pub fn get_ip_addresses(user_id: i32, max_rows: i32) -> Vec<String> {
        let mut ip_addresses = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT DISTINCT(ip_address) FROM users_ip_logs WHERE user_id = {user_id} ORDER BY created_at DESC LIMIT {max_rows}"
            ),
        ) {
            if let Some(value) = row.str("ip_address") {
                ip_addresses.push(value);
            }
        }

        ip_addresses
    }

    /// Mirrors `getLatestIp(int)`.
    pub fn get_latest_ip(user_id: i32) -> String {
        Storage::get_storage()
            .get_string(
                &format!(
                    "SELECT ip_address FROM users_ip_logs WHERE user_id = {user_id} ORDER BY created_at DESC LIMIT 1"
                ),
                "ip_address",
            )
            .unwrap_or_default()
    }

    /// Mirrors `getRandomHabbos(int)`.
    pub fn get_random_habbos(limit: i32) -> Vec<PlayerDetails> {
        let mut habbos = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM users WHERE figure NOT IN ('{FIGURE_BLACKLIST_1}') AND UNIX_TIMESTAMP(last_online) > {} ORDER BY RAND() LIMIT {limit}",
            DateUtil::get_current_time_seconds() as i64 - 86_400 * 30
        )) {
            let mut details = PlayerDetails::new();
            Self::fill(&mut details, &row);

            habbos.push(details);
        }

        habbos
    }

    /// Mirrors `getRecentHabbos(int, boolean)`.
    pub fn get_recent_habbos(limit: i32, fill_to_limit: bool) -> Vec<PlayerDetails> {
        let mut habbos = Vec::new();

        let mut total_users = 0;

        for row in Storage::get_storage().query_all("SELECT COUNT(*) as total_users FROM users") {
            if let Some(value) = row.i32("total_users") {
                total_users = value;
            }
        }

        if total_users == 0 {
            return habbos;
        }

        let mut all_users = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM users ORDER BY last_online DESC") {
            let mut details = PlayerDetails::new();
            Self::fill(&mut details, &row);

            all_users.push(details);
        }

        if all_users.len() >= limit as usize {
            habbos.extend(all_users.iter().take(limit as usize).cloned());
        } else if fill_to_limit {
            let mut users_needed = limit as usize;
            let mut current_index = 0;

            while users_needed > 0 {
                habbos.push(all_users[current_index].clone());
                users_needed -= 1;
                current_index = (current_index + 1) % all_users.len();
            }
        } else {
            habbos.extend(all_users);
        }

        habbos
    }

    /// Mirrors `getDetails(int)`.
    pub fn get_details(user_id: i32) -> Option<PlayerDetails> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users WHERE id = {user_id} LIMIT 1"))
        {
            let mut details = PlayerDetails::new();
            Self::fill(&mut details, &row);
            return Some(details);
        }

        None
    }

    /// Mirrors `getDetails(String)`.
    pub fn get_details_by_name(username: &str) -> Option<PlayerDetails> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users WHERE username = '{}' LIMIT 1", escape(username)))
        {
            let mut details = PlayerDetails::new();
            Self::fill(&mut details, &row);
            return Some(details);
        }

        None
    }

    /// Mirrors `loginTicket(Player, String)` (the Java `Player` argument is
    /// its mutable `PlayerDetails` here).
    pub fn login_ticket(details: &mut PlayerDetails, sso_ticket: &str) -> bool {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users WHERE sso_ticket = '{}' LIMIT 1", escape(sso_ticket)))
        {
            Self::fill(details, &row);
            return true;
        }

        false
    }

    /// Mirrors `login(PlayerDetails, String, String)`.
    pub fn login(details: &mut PlayerDetails, username: &str, password: &str) -> bool {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users WHERE username = '{}' LIMIT 1", escape(username)))
        {
            if let Some(database_password) = row.str("password") {
                if PlayerManager::get_instance().password_matches(&database_password, password) {
                    Self::fill(details, &row);
                    return true;
                }
            }
        }

        false
    }

    /// Mirrors `setPassword(int, String)`.
    pub fn set_password(user_id: i32, password: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET password = '{}' WHERE id = {user_id} LIMIT 1",
            escape(password)
        ));
    }

    /// Mirrors `setEmail(int, String)`.
    pub fn set_email(user_id: i32, email: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET email = '{}' WHERE id = {user_id} LIMIT 1",
            escape(email)
        ));
    }

    /// Mirrors `resetSsoTicket(int)`.
    pub fn reset_sso_ticket(user_id: i32) {
        Storage::get_storage()
            .execute(&format!("UPDATE users SET sso_ticket = NULL WHERE id = {user_id} LIMIT 1"));
    }

    /// Mirrors `resetSsoTickets()`.
    pub fn reset_sso_tickets() {
        Storage::get_storage()
            .execute("UPDATE users SET sso_ticket = NULL WHERE sso_ticket IS NOT NULL");
    }

    /// Mirrors `setTicket(int, String)`.
    pub fn set_ticket(user_id: i32, ticket: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET sso_ticket = '{}' WHERE id = {user_id} LIMIT 1",
            escape(ticket)
        ));
    }

    /// Mirrors `setMachineId(int, String)`.
    pub fn set_machine_id(user_id: i32, unique_id: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET machine_id = '{}' WHERE id = {user_id} LIMIT 1",
            escape(unique_id)
        ));
    }

    /// Mirrors `getId(String)`.
    pub fn get_id(username: &str) -> i32 {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT id FROM users WHERE LOWER(username) = '{}' LIMIT 1",
                escape(&username.to_lowercase())
            ),
        ) {
            if let Some(value) = row.i32("id") {
                return value;
            }
        }

        -1
    }

    /// Mirrors `getName(int)`.
    pub fn get_name(user_id: i32) -> Option<String> {
        Storage::get_storage().get_string(
            &format!("SELECT username FROM users WHERE id = {user_id} LIMIT 1"),
            "username",
        )
    }

    /// Mirrors `getMachineId(int)`.
    pub fn get_machine_id(user_id: i32) -> String {
        Storage::get_storage()
            .get_string(
                &format!("SELECT machine_id FROM users WHERE id = {user_id} LIMIT 1"),
                "machine_id",
            )
            .unwrap_or_default()
    }

    /// Mirrors `countMachineId(String)`.
    pub fn count_machine_id(machine_id: &str) -> i32 {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) as users_matched FROM users WHERE machine_id = '{}' LIMIT 1",
                escape(machine_id)
            ),
        ) {
            if let Some(value) = row.i32("users_matched") {
                return value;
            }
        }

        0
    }

    /// Mirrors `saveLastOnline(int, long, boolean)`.
    pub fn save_last_online(user_id: i32, last_online: i64, is_online: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET last_online = FROM_UNIXTIME({last_online}), is_online = {} WHERE id = {user_id}",
            if is_online { 1 } else { 0 }
        ));
    }

    /// Mirrors `saveSoundSetting(int, boolean)`.
    pub fn save_sound_setting(user_id: i32, sound_setting: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET sound_enabled = {} WHERE id = {user_id}",
            if sound_setting { 1 } else { 0 }
        ));
    }

    /// Mirrors `saveDetails(int, String, String, String)`.
    pub fn save_details(user_id: i32, figure: &str, pool_figure: &str, sex: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET figure = '{}', pool_figure = '{}', sex = '{}' WHERE id = {user_id}",
            escape(figure),
            escape(pool_figure),
            escape(sex)
        ));
    }

    /// Mirrors `saveMotto(int, String)`.
    pub fn save_motto(user_id: i32, motto: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET motto = '{}' WHERE id = {user_id}",
            escape(motto)
        ));
    }

    /// Mirrors `saveCurrency(int, int)`.
    pub fn save_currency(user_id: i32, credits: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET credits = {credits} WHERE id = {user_id}"
        ));
    }

    /// Mirrors `saveEmail(int, String)`.
    pub fn save_email(user_id: i32, email: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET email = '{}' WHERE id = {user_id}",
            escape(email)
        ));
    }

    /// Mirrors `saveSelectedRoom(int, int)`.
    pub fn save_selected_room(user_id: i32, selected_room: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET selected_room_id = {selected_room} WHERE id = {user_id}"
        ));
    }

    /// Mirrors `saveRespect(int, int, int, String, int)`.
    pub fn save_respect(
        user_id: i32,
        daily_respect_points: i32,
        respect_points: i32,
        respect_day: &str,
        respect_given: i32,
    ) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET daily_respect_points = {daily_respect_points}, respect_points = {respect_points}, respect_day = '{}', respect_given = {respect_given} WHERE id = {user_id}",
            escape(respect_day)
        ));
    }

    /// Mirrors `saveSubscription(int, long, long)`.
    pub fn save_subscription(user_id: i32, first_club_subscription: i64, club_expiration: i64) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET club_subscribed = {first_club_subscription}, club_expiration = {club_expiration} WHERE id = {user_id}"
        ));
    }

    /// Mirrors `saveOnlineStatus(int, boolean)`.
    pub fn save_online_status(user_id: i32, online_status_visible: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET online_status_visible = {} WHERE id = {user_id}",
            if online_status_visible { 1 } else { 0 }
        ));
    }

    /// Mirrors `saveFavouriteGroup(int, int)`.
    pub fn save_favourite_group(user_id: i32, favourite_group_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET favourite_group = {favourite_group_id} WHERE id = {user_id}"
        ));
    }

    /// Mirrors `getHomeRoom(int)`.
    pub fn get_home_room(id: i32) -> i32 {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT home_room FROM users WHERE id = {id} LIMIT 1"))
        {
            if let Some(value) = row.i32("home_room") {
                return value;
            }
        }

        0
    }

    /// Mirrors `saveHomeRoom(int, int)`.
    pub fn save_home_room(user_id: i32, room_id: i32) {
        Storage::get_storage()
            .execute(&format!("UPDATE users SET home_room = {room_id} WHERE id = {user_id}"));
    }

    /// Mirrors `isPlayerOnline(int)`.
    pub fn is_player_online(user_id: i32) -> bool {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT is_online FROM users WHERE id = {user_id} LIMIT 1"))
        {
            if let Some(value) = row.bool("is_online") {
                return value;
            }
        }

        false
    }

    /// Mirrors `getByEmail(String)`.
    pub fn get_by_email(email: &str) -> i32 {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users WHERE email = '{}' LIMIT 1", escape(email)))
        {
            if let Some(value) = row.i32("id") {
                return value;
            }
        }

        -1
    }

    /// Mirrors `savePassword(int, String)`.
    pub fn save_password(user_id: i32, password: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET password = '{}' WHERE id = {user_id}",
            escape(password)
        ));
    }

    /// Mirrors `saveBirthday(int, String)`.
    pub fn save_birthday(user_id: i32, birthday: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET birthday = '{}' WHERE id = {user_id}",
            escape(birthday)
        ));
    }

    /// Mirrors `register(String, String, String, String, String, String)`.
    pub fn register(
        username: &str,
        password: &str,
        figure: &str,
        sex: &str,
        email: &str,
        birthday: &str,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users (username, password, figure, sex, pool_figure, sso_ticket, email, birthday) VALUES ('{}', '{}', '{}', '{}', '', '', '{}', '{}')",
            escape(username),
            escape(password),
            escape(figure),
            escape(sex),
            escape(email),
            escape(birthday)
        ));
    }

    /// Mirrors `saveReceiveMail(PlayerDetails)`.
    pub fn save_receive_mail(details: &PlayerDetails) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET receive_email = {} WHERE id = {}",
            if details.is_receive_news() { 1 } else { 0 },
            details.get_id()
        ));
    }

    /// Mirrors `fill(PlayerDetails, ResultSet)`.
    pub fn fill(details: &mut PlayerDetails, row: &MySqlRow) {
        let id = row.i32("id").unwrap_or(0);
        let username = row.str("username").unwrap_or_default();
        let figure = row.str("figure").unwrap_or_default();
        let pool_figure = row.str("pool_figure").unwrap_or_default();
        let credits = row.i32("credits").unwrap_or(0);
        let email = row.str("email").unwrap_or_default();
        let motto = row.str("motto").unwrap_or_default();
        let sex = row.str("sex").unwrap_or_default();
        let sso_ticket = row.str("sso_ticket").unwrap_or_default();
        let tickets = row.i32("tickets").unwrap_or(0);
        let film = row.i32("film").unwrap_or(0);
        let rank = row.i32("rank").unwrap_or(0);
        let last_online = row.i64("last_online").unwrap_or(0);
        let join_date = row.i64("created_at").unwrap_or(0);
        let machine_id = row.str("machine_id").unwrap_or_default();
        let first_club_subscription = row.i64("club_subscribed").unwrap_or(0);
        let club_expiration = row.i64("club_expiration").unwrap_or(0);
        let allow_stalking = row.bool("allow_stalking").unwrap_or(false);
        let selected_room = row.i32("selected_room_id").unwrap_or(0);
        let allow_friend_requests = row.bool("allow_friend_requests").unwrap_or(false);
        let online_status_visible = row.bool("online_status_visible").unwrap_or(false);
        let profile_visible = row.bool("profile_visible").unwrap_or(false);
        let word_filter_enabled = row.bool("wordfilter_enabled").unwrap_or(false);
        let trade_enabled = row.bool("trade_enabled").unwrap_or(false);
        let sound_enabled = row.bool("sound_enabled").unwrap_or(false);
        let trade_ban_expiration = row.i64("trade_ban_expiration").unwrap_or(0);
        let receive_news = row.bool("receive_email").unwrap_or(false);
        let is_online = row.bool("is_online").unwrap_or(false);
        let favourite_group_id = row.i32("favourite_group").unwrap_or(0);
        let created_at = row.str("created_at").unwrap_or_default();

        details.fill(
            id,
            &username,
            &figure,
            &pool_figure,
            credits,
            &email,
            &motto,
            &sex,
            &sso_ticket,
            tickets,
            film,
            rank,
            last_online,
            join_date,
            &machine_id,
            first_club_subscription,
            club_expiration,
            allow_stalking,
            selected_room,
            allow_friend_requests,
            online_status_visible,
            profile_visible,
            word_filter_enabled,
            trade_enabled,
            sound_enabled,
            trade_ban_expiration,
            receive_news,
            is_online,
            favourite_group_id,
            &created_at,
        );
    }
}

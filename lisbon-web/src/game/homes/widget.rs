//! Mirrors `org.alexdev.http.game.homes.Widget`.

use serde::Serialize;

use lisbon_server::dao::mysql::badge_dao::BadgeDao;
use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::group_member_dao::GroupMemberDao;
use lisbon_server::dao::mysql::item_dao::ItemDao;
use lisbon_server::dao::mysql::jukebox_dao::JukeboxDao;
use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::room_dao::RoomDao;
use lisbon_server::dao::mysql::song_machine_dao::SongMachineDao;
use lisbon_server::game::badges::badge::Badge;
use lisbon_server::game::badges::badge_manager::BadgeManager;
use lisbon_server::game::groups::group::Group;
use lisbon_server::game::groups::group_member::GroupMember;
use lisbon_server::game::groups::group_member_rank::GroupMemberRank;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::game::room::room::Room;
use lisbon_server::game::room::room_manager::RoomManager;
use lisbon_server::game::song::song::Song;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::string_util::StringUtil;

use crate::dao::friend_management_dao::FriendManagementDao;
use crate::dao::guestbook_dao::GuestbookDao;
use crate::dao::rating_dao::RatingDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::web_connection::WebConnection;
use crate::game::stickers::sticker_manager::StickerManager;
use crate::game::stickers::sticker_product::StickerProduct;
use crate::game::stickers::sticker_type::StickerType;
use crate::template::twig_template::TwigTemplate;
use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

/// Mirrors `FRIENDS_PAGING_AMOUNT`.
const FRIENDS_PAGING_AMOUNT: i32 = 32;

/// Mirrors `MEMBER_PAGING_AMOUNT` (unused in Java as well).
#[allow(dead_code)]
const MEMBER_PAGING_AMOUNT: i32 = 32;

#[derive(Clone, Debug, Serialize)]
pub struct Widget {
    pub id: i32,
    pub user_id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub sticker_id: i32,
    pub skin_id: i32,
    pub group_id: i32,
    pub amount: i32,
    pub text: String,
    pub is_placed: bool,
    pub extra_data: Option<String>,
}

impl Widget {
    /// Mirrors the `Widget(int, int, int, int, int, int, int, int, String, int, boolean, String)` constructor
    /// (`extraData` is `Option<String>`).
    pub fn new(
        id: i32,
        user_id: i32,
        x: i32,
        y: i32,
        z: i32,
        sticker_id: i32,
        skin_id: i32,
        group_id: i32,
        text: &str,
        amount: i32,
        is_placed: bool,
        extra_data: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            x,
            y,
            z,
            sticker_id,
            skin_id,
            group_id,
            amount,
            text: text.to_string(),
            is_placed,
            extra_data,
        }
    }

    /// Mirrors `template(WebConnection)`.
    pub fn template<'a>(&self, web_connection: &'a WebConnection) -> TwigTemplate<'a> {
        let data = self
            .get_product()
            .map(|product| product.data.to_lowercase())
            .unwrap_or_default();

        let mut tpl = match data.as_str() {
            "groupinfowidget" => web_connection.template("homes/widget/group_info_widget"),
            "guestbookwidget" => web_connection.template("homes/widget/guestbook_widget"),
            "stickienote" => web_connection.template("homes/widget/note"),
            "memberwidget" => web_connection.template("homes/widget/member_widget"),
            "traxplayerwidget" => web_connection.template("homes/widget/trax_player_widget"),
            "profilewidget" => web_connection.template("homes/widget/profile_widget"),
            "roomswidget" => web_connection.template("homes/widget/rooms_widget"),
            "highscoreswidget" => web_connection.template("homes/widget/highscores_widget"),
            "badgeswidget" => web_connection.template("homes/widget/badges_widget"),
            "friendswidget" => web_connection.template("homes/widget/friends_widget"),
            "groupswidget" => web_connection.template("homes/widget/groups_widget"),
            "ratingwidget" => web_connection.template("homes/widget/rating_widget"),
            _ => web_connection.template("homes/widget/sticker"),
        };

        tpl.set(
            "editMode",
            TemplateValue::json(serde_json::json!(
                web_connection.session().contains("homeEditSession")
                    || web_connection.session().contains("groupEditSession")
            )),
        );
        tpl.set("sticker", TemplateValue::of(self));

        if web_connection.session().contains("homeEditSession") {
            if let Some(player_details) =
                PlayerDao::get_details(web_connection.session().get_int("user.id"))
            {
                tpl.set("user", TemplateValue::of(player_details.clone()));
                tpl.set("canAddFriend", TemplateValue::json(serde_json::json!(false)));

                if data == "profilewidget" {
                    let badges = BadgeDao::get_badges(player_details.get_id());
                    let enabled_badges = badges.clone();

                    let mut equipped: Vec<Badge> = badges
                        .into_iter()
                        .filter(|badge| badge.is_equipped())
                        .collect();
                    equipped.sort_by_key(|badge| badge.get_slot_id());
                    let _ = equipped;

                    tpl.set(
                        "hasBadge",
                        TemplateValue::json(serde_json::json!(enabled_badges.len() > 0)),
                    );

                    if enabled_badges.len() > 0 {
                        tpl.set(
                            "badgeCode",
                            TemplateValue::of(enabled_badges[0].get_badge_code()),
                        );
                    }

                    tpl.set("hasFavouriteGroup", TemplateValue::json(serde_json::json!(false)));

                    if player_details.get_favourite_group_id() > 0
                        && GroupDao::get_group(player_details.get_favourite_group_id()).is_some()
                    {
                        tpl.set(
                            "hasFavouriteGroup",
                            TemplateValue::json(serde_json::json!(true)),
                        );
                        tpl.set(
                            "group",
                            TemplateValue::of(
                                GroupDao::get_group(player_details.get_favourite_group_id())
                                    .unwrap(),
                            ),
                        );
                    }
                }
            }
        }

        tpl
    }

    /// Mirrors `getFirstBadges()`.
    pub fn get_first_badges(&self) -> Vec<Badge> {
        let badges = BadgeManager::new(self.user_id).get_badges();
        StringUtil::paginate_ext(&badges, 16, true)
            .get(&0)
            .cloned()
            .unwrap_or_default()
    }

    /// Mirrors `getBadgeList()`.
    pub fn get_badge_list(&self) -> std::collections::HashMap<usize, Vec<Badge>> {
        let badges = BadgeManager::new(self.user_id).get_badges();
        StringUtil::paginate(&badges, 16)
    }

    /// Mirrors `getOwnerRooms()`.
    pub fn get_owner_rooms(&self) -> Vec<Room> {
        let mut room_list = RoomDao::get_rooms_by_user_id(self.user_id);
        RoomManager::get_instance().sort_rooms(&mut room_list);

        room_list
    }

    /// Mirrors `getFirstFriendsList()`.
    pub fn get_first_friends_list(&self) -> Vec<lisbon_server::game::messenger::messenger_user::MessengerUser> {
        FriendManagementDao::get_friends(self.user_id, 1, FRIENDS_PAGING_AMOUNT)
    }

    /// Mirrors `getFriendsPagesSearch(String)`.
    pub fn get_friends_pages_search(&self, search: &str) -> i32 {
        let friends = FriendManagementDao::get_friends_count_search(self.user_id, search);
        let pages = if friends > 0 {
            (friends as f64 / FRIENDS_PAGING_AMOUNT as f64).ceil() as i32
        } else {
            0
        };

        if pages == 0 { 1 } else { pages }
    }

    /// Mirrors `getFriendsPages()`.
    pub fn get_friends_pages(&self) -> i32 {
        let friends = FriendManagementDao::get_friends_count(self.user_id);
        let pages = if friends > 0 {
            (friends as f64 / FRIENDS_PAGING_AMOUNT as f64).ceil() as i32
        } else {
            0
        };

        if pages == 0 { 1 } else { pages }
    }

    /// Mirrors `getFriendsAmount()`.
    pub fn get_friends_amount(&self) -> i32 {
        MessengerDao::get_friends_count(self.user_id)
    }

    /// Mirrors `getFriendsList(String, int)`.
    pub fn get_friends_list(
        &self,
        query: &str,
        page: i32,
    ) -> Vec<lisbon_server::game::messenger::messenger_user::MessengerUser> {
        if !query.trim().is_empty() {
            FriendManagementDao::get_friends_search(self.user_id, query, page, FRIENDS_PAGING_AMOUNT)
        } else {
            FriendManagementDao::get_friends(self.user_id, page, FRIENDS_PAGING_AMOUNT)
        }
    }

    /// Mirrors `getGuestbookState()`.
    pub fn get_guestbook_state(&self) -> String {
        match self.extra_data.as_deref().unwrap_or_default().to_lowercase().as_str() {
            "public" => "public".to_string(),
            "private" => "private".to_string(),
            _ => "public".to_string(),
        }
    }

    /// Mirrors `isPostingAllowed(int)`.
    pub fn is_posting_allowed(&self, user_id: i32) -> bool {
        if self.get_guestbook_state() == "public" {
            return true;
        }

        let Some(product) = self.get_product() else {
            return false;
        };

        if product.get_type() == Some(StickerType::GroupWidget) {
            let Some(group) = GroupDao::get_group(self.group_id) else {
                return false;
            };

            return group.is_member(user_id);
        }

        if product.get_type() == Some(StickerType::HomeWidget) {
            return user_id == self.user_id || MessengerDao::friend_exists(user_id, self.user_id);
        }

        false
    }

    /// Mirrors `getGuestbookEntries()`.
    pub fn get_guestbook_entries(&self) -> Vec<crate::game::homes::guestbook_entry::GuestbookEntry> {
        match self.get_product().map(|product| product.get_type()) {
            Some(Some(StickerType::GroupWidget)) => GuestbookDao::get_entries_by_group(self.group_id),
            Some(Some(StickerType::HomeWidget)) => GuestbookDao::get_entries_by_home(self.user_id),
            _ => Vec::new(),
        }
    }

    /// Mirrors `canDeleteEntries(int)`.
    pub fn can_delete_entries(&self, user_id: i32) -> bool {
        match self.get_product().map(|product| product.get_type()) {
            Some(Some(StickerType::GroupWidget)) => {
                GroupDao::get_group_owner(self.group_id) == user_id
            }
            Some(Some(StickerType::HomeWidget)) => user_id == self.user_id,
            _ => false,
        }
    }

    /// Mirrors `getSongs()`.
    pub fn get_songs(&self) -> Vec<Song> {
        let user_id = match self.get_product().map(|product| product.get_type()) {
            Some(Some(StickerType::GroupWidget)) => GroupDao::get_group_owner(self.group_id),
            Some(Some(StickerType::HomeWidget)) => self.user_id,
            _ => return Vec::new(),
        };

        let mut song_list = SongMachineDao::get_song_user_list(user_id);

        let Some(definition) = ItemManager::get_instance().get_definition_by_sprite("song_disk") else {
            return song_list;
        };

        for item in ItemDao::get_user_items_by_definition(user_id, &definition) {
            let song_id = JukeboxDao::get_song_id_by_item(item.get_id() as i64);
            let Some(song) = SongMachineDao::get_song(song_id) else {
                continue;
            };

            if !song_list.iter().any(|existing| existing.get_id() == song.get_id()) {
                song_list.push(song);
            }
        }

        song_list
    }

    /// Mirrors `getFirstMembersList()`.
    pub fn get_first_members_list(&self) -> Vec<GroupMember> {
        let mut members = GroupMemberDao::get_members(
            self.group_id,
            false,
            "",
            1,
            FRIENDS_PAGING_AMOUNT,
        );
        members.push(GroupMember::new(
            GroupDao::get_group_owner(self.group_id),
            self.group_id,
            false,
            GroupMemberRank::Owner.get_rank_id(),
        ));
        members.sort_by(|a, b| {
            let a_last_online = a
                .get_user()
                .map(|user| user.get_last_online())
                .unwrap_or(0);
            let b_last_online = b
                .get_user()
                .map(|user| user.get_last_online())
                .unwrap_or(0);
            b_last_online.cmp(&a_last_online)
        });
        members
    }

    /// Mirrors `getMembersList(String, int)`.
    pub fn get_members_list(&self, query: &str, page: i32) -> Vec<GroupMember> {
        let mut members = GroupMemberDao::get_members(
            self.group_id,
            false,
            query,
            page,
            FRIENDS_PAGING_AMOUNT,
        );
        members.sort_by(|a, b| {
            let a_last_online = a
                .get_user()
                .map(|user| user.get_last_online())
                .unwrap_or(0);
            let b_last_online = b
                .get_user()
                .map(|user| user.get_last_online())
                .unwrap_or(0);
            b_last_online.cmp(&a_last_online)
        });
        members
    }

    /// Mirrors `getMembersAmount()`.
    pub fn get_members_amount(&self) -> i32 {
        GroupMemberDao::count_members(self.group_id, false) + 1
    }

    /// Mirrors `getMembersPages()`.
    pub fn get_members_pages(&self) -> i32 {
        let members = GroupMemberDao::count_members(self.group_id, false) + 1;
        let pages = if members > 0 {
            (members as f64 / FRIENDS_PAGING_AMOUNT as f64).ceil() as i32
        } else {
            0
        };

        if pages == 0 { 1 } else { pages }
    }

    /// Mirrors `hasSong()`.
    pub fn has_song(&self) -> bool {
        self.get_song().is_some()
    }

    /// Mirrors `getSong()`.
    // Port note: a null `extraData` (Java NPE-free via `StringUtils.isNumeric`
    // returning false for null) degrades to the empty string.
    pub fn get_song(&self) -> Option<Song> {
        let data = self.extra_data.as_deref().unwrap_or_default();

        if !data.is_empty() && data.bytes().all(|byte| byte.is_ascii_digit()) {
            return SongMachineDao::get_song(data.parse().unwrap_or(0));
        }

        None
    }

    /// Mirrors `getOwnerGroups()`.
    pub fn get_owner_groups(&self) -> Vec<Group> {
        GroupDao::get_joined_groups(self.user_id)
    }

    /// Mirrors `hasRated(int)`.
    pub fn has_rated(&self, user_id: i32) -> bool {
        RatingDao::has_rated(user_id, self.user_id)
    }

    /// Mirrors `getAverageRating()`.
    pub fn get_average_rating(&self) -> i32 {
        RatingDao::get_average_rating(self.user_id) as i32
    }

    /// Mirrors `getRatingPixels()`.
    pub fn get_rating_pixels(&self) -> i32 {
        let mut rating = self.get_average_rating() as f64;

        if rating <= 0.0 {
            rating = 1.0;
        }

        (rating * 150.0 / 5.0).round() as i32
    }

    /// Mirrors `getHighVoteCount()`.
    pub fn get_high_vote_count(&self) -> i32 {
        RatingDao::get_high_vote_count(self.user_id)
    }

    /// Mirrors `getVoteCount()`.
    pub fn get_vote_count(&self) -> i32 {
        RatingDao::get_vote_count(self.user_id)
    }

    /// Mirrors `save()`.
    pub fn save(&self) {
        WidgetDao::save(self);
    }

    /// Mirrors `getProduct()`.
    pub fn get_product(&self) -> Option<StickerProduct> {
        StickerManager::get_instance().get_sticker_product(self.sticker_id)
    }

    /// Mirrors `getSkin()`.
    pub fn get_skin(&self) -> String {
        StickerManager::get_instance().get_skin(self.skin_id)
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `setX(int)`.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `setY(int)`.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Mirrors `getZ()`.
    pub fn get_z(&self) -> i32 {
        self.z
    }

    /// Mirrors `setZ(int)`.
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }

    /// Mirrors `setSkinId(int)`.
    pub fn set_skin_id(&mut self, skin_id: i32) {
        self.skin_id = skin_id;
    }

    /// Mirrors `setGroupId(int)`.
    pub fn set_group_id(&mut self, group_id: i32) {
        self.group_id = group_id;
    }

    /// Mirrors `setAmount(int)`.
    pub fn set_amount(&mut self, amount: i32) {
        self.amount = amount;
    }

    /// Mirrors `getText()`.
    pub fn get_text(&self) -> String {
        WordfilterManager::filter_sentence(&self.text)
    }

    /// Mirrors `getFormattedText()`.
    pub fn get_formatted_text(&self) -> String {
        BBCode::format(
            &HtmlUtil::escape(&BBCode::normalise(&WordfilterManager::filter_sentence(&self.text))),
            false,
        )
    }

    /// Mirrors `setText(String)`.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Mirrors `setPlaced(boolean)`.
    pub fn set_placed(&mut self, placed: bool) {
        self.is_placed = placed;
    }

    /// Mirrors `setExtraData(String)`.
    pub fn set_extra_data(&mut self, extra_data: Option<String>) {
        self.extra_data = extra_data;
    }
}

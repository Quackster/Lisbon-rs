//! Mirrors `net.h4bbo.lisbon.game.tags.HabboTag`.

use crate::dao::mysql::group_dao::GroupDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::tag_dao::TagDao;
use crate::game::groups::group::Group;
use crate::game::player::player_details::PlayerDetails;

#[derive(Clone, serde::Serialize)]
pub struct HabboTag {
    tag: String,
    room_id: i32,
    user_id: i32,
    group_id: i32,
    group_data: Option<Group>,
    user_data: Option<PlayerDetails>,
    tag_list: Vec<String>,
}

impl HabboTag {
    /// Mirrors the 4-arg `HabboTag(String, int, int, int)` constructor.
    pub fn new(tag: String, room_id: i32, user_id: i32, group_id: i32) -> Self {
        Self {
            tag,
            room_id,
            user_id,
            group_id,
            group_data: None,
            user_data: None,
            tag_list: Vec::new(),
        }
    }

    /// Mirrors `getTagList()`.
    pub fn get_tag_list(&mut self) -> Vec<String> {
        if self.group_id > 0 {
            self.get_group_data();
        }

        if self.user_id > 0 {
            self.get_user_data();
        }

        self.tag_list.clone()
    }

    /// Mirrors `getTag()`.
    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    /// Mirrors `getRoomId()`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getGroupId()`.
    pub fn get_group_id(&self) -> i32 {
        self.group_id
    }

    /// Mirrors `getGroupData()`.
    pub fn get_group_data(&mut self) -> Option<&Group> {
        if self.group_data.is_none() && self.group_id > 0 {
            self.group_data = GroupDao::get_group(self.group_id);
            self.tag_list = TagDao::get_group_tags(self.group_id);
        }

        self.group_data.as_ref()
    }

    /// Mirrors `getUserData()`.
    pub fn get_user_data(&mut self) -> Option<&PlayerDetails> {
        if self.user_data.is_none() && self.user_id > 0 {
            self.user_data = PlayerDao::get_details(self.user_id);
            self.tag_list = TagDao::get_user_tags(self.user_id);
        }

        self.user_data.as_ref()
    }
}

//! Mirrors `net.h4bbo.lisbon.game.room.RoomData`.
//!
//! Java holds a back-reference to its `Room`; Rust has no reference
//! cycles, so the methods that need the room take it as an explicit
//! parameter.

use crate::game::games::game::Game;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
pub struct RoomData {
    id: i32,
    owner_id: i32,
    owner_name: String,
    category_id: i32,
    name: String,
    description: String,
    model: String,
    ccts: String,
    wallpaper: i32,
    floor: i32,
    landscape: String,
    show_owner_name: bool,
    super_users: bool,
    is_game_arena: bool,
    game_lobby: Option<String>,
    access_type: i32,
    password: Option<String>,
    visitors_now: i32,
    visitors_max: i32,
    rating: i32,
    game: Option<Game>,
    group_id: i32,
    is_hidden: bool,
}

impl RoomData {
    /// Mirrors the `RoomData(Room)` constructor (the back-reference is
    /// omitted, see module note).
    pub fn new() -> Self {
        Self {
            id: 0,
            owner_id: 0,
            owner_name: String::new(),
            category_id: 0,
            name: String::new(),
            description: String::new(),
            model: String::new(),
            ccts: String::new(),
            wallpaper: 0,
            floor: 0,
            landscape: String::new(),
            show_owner_name: false,
            super_users: false,
            is_game_arena: false,
            game_lobby: None,
            access_type: 0,
            password: None,
            visitors_now: 0,
            visitors_max: 0,
            rating: 0,
            game: None,
            group_id: 0,
            is_hidden: false,
        }
    }

    /// Mirrors the 3-arg `fill(int, String, String)` overload.
    pub fn fill_brief(&mut self, id: i32, name: &str, description: &str) {
        self.id = id;
        self.name = name.to_string();
        self.description = description.to_string();
        self.owner_id = 0;
        self.ccts = String::new();
        self.model = String::new();
        self.owner_name = String::new();
    }

    /// Mirrors the 19-arg `fill` overload.
    pub fn fill(
        &mut self,
        room: &Room,
        id: i32,
        owner_id: i32,
        owner_name: &str,
        category: i32,
        name: &str,
        description: &str,
        model: &str,
        ccts: &str,
        wallpaper: i32,
        floor: i32,
        landscape: &str,
        show_name: bool,
        super_users: bool,
        access_type: i32,
        password: Option<&str>,
        visitors_now: i32,
        visitors_max: i32,
        rating: i32,
        group_id: i32,
        is_hidden: bool,
    ) {
        self.id = id;
        self.owner_id = owner_id;
        self.owner_name = StringUtil::filter_input(owner_name, true);
        self.category_id = category;
        self.name = StringUtil::filter_input(name, true);
        self.description = StringUtil::filter_input(description, true);
        self.model = model.to_string();
        self.ccts = ccts.to_string();
        self.wallpaper = wallpaper;
        self.floor = floor;
        self.landscape = landscape.to_string();
        self.show_owner_name = show_name;
        self.super_users = super_users;
        self.access_type = access_type;
        self.password = password.map(String::from);
        self.visitors_now = visitors_now;
        self.visitors_max = visitors_max;
        self.rating = rating;
        self.group_id = group_id;
        self.is_hidden = is_hidden;

        self.apply_walkway_follow_redirect(room);
    }

    /// Mirrors the walkway follow-redirect lookup at the end of the Java
    /// `fill`.
    pub fn apply_walkway_follow_redirect(&self, room: &Room) {
        let walkways = WalkwaysManager::get_instance().get_walkways();

        if walkways
            .iter()
            .any(|walkway| walkway.get_room_target_id() == room.get_id())
        {
            if let Some(walkway) = walkways
                .iter()
                .find(|walkway| walkway.get_room_target_id() == room.get_id())
            {
                room.set_follow_redirect(walkway.get_room_id());
            }
        }
    }

    /// Mirrors `isNavigatorHide`.
    pub fn is_navigator_hide(&self) -> bool {
        if GameConfiguration::get_instance().get_bool("navigator.show.hidden.rooms") {
            return false;
        }

        self.is_hidden
    }

    /// Mirrors `getTotalVisitorsNow`.
    pub fn get_total_visitors_now(&self, room: &Room) -> i32 {
        let mut total_visitors = self.get_visitors_now();

        for child_room in RoomManager::get_instance().get_child_rooms(room) {
            total_visitors += child_room.lock().get_data().get_visitors_now();
        }

        total_visitors
    }

    /// Mirrors `getTotalVisitorsMax`.
    pub fn get_total_visitors_max(&self, room: &Room) -> i32 {
        let mut total_max_visitors = self.get_visitors_max();

        for child_room in RoomManager::get_instance().get_child_rooms(room) {
            total_max_visitors += child_room.lock().get_data().get_visitors_max();
        }

        total_max_visitors
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getOwnerId`.
    pub fn get_owner_id(&self) -> i32 {
        self.owner_id
    }

    /// Mirrors `setOwnerId`.
    pub fn set_owner_id(&mut self, owner_id: i32) {
        self.owner_id = owner_id;
    }

    /// Mirrors `getOwnerName`.
    pub fn get_owner_name(&self) -> &str {
        &self.owner_name
    }

    /// Mirrors `getCategoryId`.
    pub fn get_category_id(&self) -> i32 {
        self.category_id
    }

    /// Mirrors `setCategoryId`.
    pub fn set_category_id(&mut self, category_id: i32) {
        self.category_id = category_id;
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getPublicName` (Java consults `room.isPublicRoom()`).
    pub fn get_public_name(&self, room: &Room) -> String {
        if room.is_public_room() {
            if self.name.starts_with("Upper Hallways") {
                return "Upper Hallways".to_string();
            }

            if self.name.starts_with("Lower Hallways") {
                return "Lower Hallways".to_string();
            }

            if self.name.starts_with("Club Massiva") {
                return "Club Massiva".to_string();
            }

            if self.name.starts_with("The Chromide Club") {
                return "The Chromide Club".to_string();
            }

            if self.ccts == "hh_room_gamehall,hh_games" {
                return "Cunning Fox Gamehall".to_string();
            }
        }

        self.name.clone()
    }

    /// Mirrors `setName`.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Mirrors `getDescription`.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Mirrors `setDescription`.
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    /// Mirrors `getModel`.
    pub fn get_model(&self) -> &str {
        &self.model
    }

    /// Mirrors `getCcts`.
    pub fn get_ccts(&self) -> &str {
        &self.ccts
    }

    /// Mirrors `setCcts`.
    pub fn set_ccts(&mut self, ccts: &str) {
        self.ccts = ccts.to_string();
    }

    /// Mirrors `getWallpaper`.
    pub fn get_wallpaper(&self) -> i32 {
        self.wallpaper
    }

    /// Mirrors `setWallpaper`.
    pub fn set_wallpaper(&mut self, wallpaper: i32) {
        self.wallpaper = wallpaper;
    }

    /// Mirrors `getFloor`.
    pub fn get_floor(&self) -> i32 {
        self.floor
    }

    /// Mirrors `setFloor`.
    pub fn set_floor(&mut self, floor: i32) {
        self.floor = floor;
    }

    /// Mirrors `getLandscape`.
    pub fn get_landscape(&self) -> &str {
        &self.landscape
    }

    /// Mirrors `setLandscape`.
    pub fn set_landscape(&mut self, landscape: &str) {
        self.landscape = landscape.to_string();
    }

    /// Mirrors `showOwnerName`.
    pub fn show_owner_name(&self) -> bool {
        self.show_owner_name
    }

    /// Mirrors `setShowOwnerName`.
    pub fn set_show_owner_name(&mut self, show_name: bool) {
        self.show_owner_name = show_name;
    }

    /// Mirrors `allowSuperUsers`.
    pub fn allow_super_users(&self) -> bool {
        self.super_users
    }

    /// Mirrors `setSuperUsers`.
    pub fn set_super_users(&mut self, super_users: bool) {
        self.super_users = super_users;
    }

    /// Mirrors `getAccessType`.
    pub fn get_access_type(&self) -> &'static str {
        if self.access_type == 2 {
            return "password";
        }

        if self.access_type == 1 {
            return "closed";
        }

        "open"
    }

    /// Mirrors `getAccessTypeId`.
    pub fn get_access_type_id(&self) -> i32 {
        self.access_type
    }

    /// Mirrors `setAccessType`.
    pub fn set_access_type(&mut self, access_type: i32) {
        self.access_type = access_type;
    }

    /// Mirrors `getPassword`.
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Mirrors `setPassword`.
    pub fn set_password(&mut self, password: Option<&str>) {
        self.password = password.map(String::from);
    }

    /// Mirrors `getVisitorsNow`.
    pub fn get_visitors_now(&self) -> i32 {
        self.visitors_now
    }

    /// Mirrors `setVisitorsNow`.
    pub fn set_visitors_now(&mut self, visitors_now: i32) {
        self.visitors_now = visitors_now;
    }

    /// Mirrors `getVisitorsMax`.
    pub fn get_visitors_max(&self) -> i32 {
        self.visitors_max
    }

    /// Mirrors `setVisitorsMax`.
    pub fn set_visitors_max(&mut self, visitors_max: i32) {
        self.visitors_max = visitors_max;
    }

    /// Mirrors `getRating`.
    pub fn get_rating(&self) -> i32 {
        self.rating
    }

    /// Mirrors `setRating`.
    pub fn set_rating(&mut self, amount: i32) {
        self.rating = amount;
    }

    /// Mirrors `isGameArena`.
    pub fn is_game_arena(&self) -> bool {
        self.is_game_arena
    }

    /// Mirrors `setGameArena`.
    pub fn set_game_arena(&mut self, game_arena: bool) {
        self.is_game_arena = game_arena;
    }

    /// Mirrors `getGameLobby`.
    pub fn get_game_lobby(&self) -> Option<&str> {
        self.game_lobby.as_deref()
    }

    /// Mirrors `setGameLobby`.
    pub fn set_game_lobby(&mut self, game_lobby: Option<&str>) {
        self.game_lobby = game_lobby.map(String::from);
    }

    /// Mirrors `getGame`.
    pub fn get_game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    /// Mirrors `setGame`.
    pub fn set_game(&mut self, game: Option<Game>) {
        self.game = game;
    }

    /// Mirrors `isHidden`.
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    /// Mirrors `setHidden`.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.is_hidden = hidden;
    }

    /// Mirrors `getGroupId`.
    pub fn get_group_id(&self) -> i32 {
        self.group_id
    }

    /// Mirrors `setGroupId`.
    pub fn set_group_id(&mut self, group_id: i32) {
        self.group_id = group_id;
    }
}

impl Default for RoomData {
    fn default() -> Self {
        Self::new()
    }
}

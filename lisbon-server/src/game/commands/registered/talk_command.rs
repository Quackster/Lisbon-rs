//! Mirrors `net.h4bbo.lisbon.game.commands.registered.TalkCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::items::place_flooritem::PLACE_FLOORITEM;
use crate::util::string_util::StringUtil;

/// Mirrors `TalkCommand`.
pub struct TalkCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl TalkCommand {
    /// Mirrors `createVoiceSpeakMessage(Room, String)`.
    pub fn create_voice_speak_message(room: &Room, text: &str) {
        // 'Speaker'
        let mut p_item = Item::new();
        p_item.set_id(i32::MAX);
        *p_item.get_position_mut() = Position::new(255, 255, -1.0);
        p_item.set_custom_data(&format!("voiceSpeak(\"{}\")", text));
        p_item.get_definition_mut().set_sprite("spotlight");
        p_item.get_definition_mut().set_length(1);
        p_item.get_definition_mut().set_width(1);
        room.send(&PLACE_FLOORITEM::new(Box::new(p_item)));
    }
}

impl Command for TalkCommand {
    /// Mirrors the no-arg constructor.
    fn new() -> Self {
        let mut this = Self {
            permissions: Vec::new(),
            arguments: Vec::new(),
        };
        this.add_permissions();
        this.add_arguments();
        this
    }

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self) {
        self.permissions.push(Fuseright::AdministratorAccess);
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let talk_message = if !args.is_empty() {
            StringUtil::filter_input(&args.join(" "), true)
        } else {
            String::new()
        };

        let Some(room) = player.get_room_user().and_then(crate::game::room::entities::room_entity::RoomEntity::get_room) else {
            return;
        };

        Self::create_voice_speak_message(&room, &talk_message);
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Voice to text command".to_string()
    }

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright> {
        self.permissions.clone()
    }

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}

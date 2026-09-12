//! Mirrors `net.h4bbo.lisbon.game.commands.registered.RainbowDimmerCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::tasks::rainbow_task::RainbowTask;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};

/// Mirrors `RainbowDimmerCommand`.
pub struct RainbowDimmerCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for RainbowDimmerCommand {
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
        self.permissions.push(Fuseright::Default);
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let Some(room_user) = player.get_room_user() else {
            return;
        };
        let Some(room) = room_user.get_room() else {
            return;
        };

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return;
        }

        let mut tick_interval = 5;

        if args.len() == 1 {
            if !args[0].chars().all(|c| c.is_ascii_digit()) {
                player.send(&ALERT::new(
                    "Please specify the amount of seconds inbetween the colours changing as a number",
                ));
                return;
            } else {
                tick_interval = args[0].parse().unwrap_or(5);
            }
        }

        if tick_interval < 1 {
            tick_interval = 1;
        }

        let moodlight = room.get_item_manager().get_moodlight();

        if moodlight.is_none() {
            player.send(&ALERT::new(
                "This command requires a moodlight placed for it to work",
            ));
            return;
        }

        let Some(room_owner) =
            PlayerManager::get_instance().get_player_by_id(room.get_data().get_owner_id())
        else {
            return;
        };

        let room_owner = room_owner.lock();
        let owner_in_room = room_owner
            .get_room_user()
            .and_then(crate::game::room::entities::room_entity::RoomEntity::get_room)
            .map(|room| room.get_data().get_owner_id())
            .unwrap_or(0)
            == room.get_data().get_owner_id();

        let status_message;

        if room.get_task_manager().has_task("RainbowTask") {
            room.get_task_manager().cancel_task("RainbowTask");

            status_message = "Rainbow room dimmer cycle has stopped";
        } else {
            let rainbow_task = RainbowTask::new(&room);
            room.get_task_manager()
                .schedule_task("RainbowTask", std::sync::Arc::new(rainbow_task), 0, tick_interval);

            status_message = "Rainbow room dimmer cycle has started";
        }

        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            room_user.get_instance_id(),
            status_message,
        ));

        // Send status of room task to roomowner
        if owner_in_room {
            room_owner.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_owner
                    .get_room_user()
                    .map(crate::game::room::entities::room_entity::RoomEntity::get_instance_id)
                    .unwrap_or(0),
                status_message,
            ));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "<seconds> - Cycles through the rainbow in your very own room!".to_string()
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

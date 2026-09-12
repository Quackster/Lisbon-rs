//! Mirrors `net.h4bbo.lisbon.game.commands.registered.GiveDrinkCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::enums::status_type::StatusType;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};

/// Mirrors `GiveDrinkCommand`.
pub struct GiveDrinkCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for GiveDrinkCommand {
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

    /// Mirrors `addArguments()`.
    fn add_arguments(&mut self) {
        self.arguments.push("user".to_string());
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
        let Some(player_room) = room_user.get_room() else {
            return;
        };

        let Some(target_user) = PlayerManager::get_instance().get_player_by_name(&args[0]) else {
            player.send(&ALERT::new(&format!("Could not find user: {}", args[0])));
            return;
        };

        let target_user = target_user.lock();

        let Some(target_room) = target_user.get_room_user().and_then(crate::game::room::entities::room_entity::RoomEntity::get_room) else {
            player.send(&ALERT::new(&format!("Could not find user: {}", args[0])));
            return;
        };

        if target_room.get_id() != player_room.get_id() {
            player.send(&ALERT::new(&format!("Could not find user: {}", args[0])));
            return;
        }

        if !room_user.contains_status(StatusType::CarryDrink)
            && !room_user.contains_status(StatusType::CarryFood)
        {
            player.send(&ALERT::new(
                "You are not carrying any food or drinks to give.",
            ));
            return;
        }

        let mut drink = None;

        if room_user.contains_status(StatusType::CarryDrink) {
            drink = room_user.get_status(StatusType::CarryDrink);
        }

        if room_user.contains_status(StatusType::CarryFood) {
            drink = room_user.get_status(StatusType::CarryFood);
        }

        if let Some(drink) = drink {
            if target_user
                .get_room_user()
                .map(|ru| ru.contains_status(StatusType::AvatarSleep))
                .unwrap_or(false)
            {
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    &format!(
                        "{} is sleeping.",
                        target_user.get_details().get_name()
                    ),
                ));
                return;
            }

            // Give drink to user if they're not already having a drink or food, and they're not dancing
            if target_user
                .get_room_user()
                .map(|ru| {
                    ru.contains_status(StatusType::CarryFood)
                        || ru.contains_status(StatusType::CarryDrink)
                })
                .unwrap_or(false)
            {
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    &format!(
                        "{} is already enjoying a drink.",
                        target_user.get_details().get_name()
                    ),
                ));
                return;
            }

            if target_user
                .get_room_user()
                .map(|ru| ru.contains_status(StatusType::Dance))
                .unwrap_or(false)
            {
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    &format!(
                        "Can't hand drink to {}, because he/she is dancing.",
                        target_user.get_details().get_name()
                    ),
                ));
                return;
            }

            let carry_id = drink.get_value().parse::<i32>().unwrap_or(0);
            if let Some(target_room_user) = target_user.get_room_user() {
                target_room_user.carry_item(carry_id, None);
            }
            let carry_name =
                TextsManager::get_instance().get_value(&format!("handitem{}", carry_id));

            target_user.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                target_user.get_room_user().map(crate::game::room::entities::room_entity::RoomEntity::get_instance_id).unwrap_or(0),
                &format!(
                    "{} handed you a {}.",
                    player.get_details().get_name(),
                    carry_name
                ),
            ));

            room_user.remove_status(StatusType::CarryDrink);
            room_user.remove_status(StatusType::CarryFood);
            room_user.set_needs_update(true);
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Gives a user your own drink".to_string()
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
